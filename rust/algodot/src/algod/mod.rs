use algodot_core::*;
use algodot_macros::*;
use algonaut::algod::{v2::Algod, AlgodBuilder, AlgodCustomEndpointBuilder};
use algonaut::core::{CompiledTealBytes, MicroAlgos, Round};
use algonaut::model::algod::v2::{PendingTransaction, TransactionResponse};
use algonaut::transaction::transaction::{
    ApplicationCallOnComplete, ApplicationCallTransaction, AssetConfigurationTransaction,
    AssetParams, AssetTransferTransaction, StateSchema,
};
use algonaut::transaction::tx_group::TxGroup;
use algonaut::transaction::{Pay, TransactionType, TxnBuilder};
use gdnative::api::{Engine, JSONParseResult, JSON};
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};
use serde::Serialize;
use std::rc::Rc;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Algodot {
    #[property(after_set = "Self::update_algod")]
    url: String,

    #[property(after_set = "Self::update_algod")]
    token: String,

    #[property(after_set = "Self::update_algod")]
    headers: StringArray,

    algod: Rc<Algod>,
}

impl Algodot {
    fn new(_owner: TRef<Node>) -> Self {
        Algodot {
            url: String::new(),
            token: String::new(),
            headers: StringArray::new(),

            // algod will be initialised on _enter_tree()
            // leave these default values here for now
            algod: Rc::new(
                AlgodBuilder::new()
                    .bind("http://localhost:4001")
                    .auth("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
                    .build_v2()
                    .unwrap(),
            ),
        }
    }

    fn register(builder: &ClassBuilder<Algodot>) {
        Self::register_signals(builder);

        // made with asyncmethods! macro
        register_methods(builder);
    }

    fn register_signals(builder: &ClassBuilder<Algodot>) {
        builder.add_signal(Signal {
            name: "transaction_confirmed",
            args: &[SignalArgument {
                name: "transaction_info",
                default: ().to_variant(),
                export_info: ExportInfo::new(VariantType::Dictionary),
                usage: PropertyUsage::DEFAULT,
            }],
        })
    }

    async fn wait_for_transaction(
        algod: Rc<Algod>,
        tx: TransactionResponse,
    ) -> Result<PendingTransaction, AlgodotError> {
        let status = algod.status().await?;
        let mut round = status.last_round;
        loop {
            // wait for next round
            round += 1;
            algod.status_after_round(Round(round)).await?;
            let txn = algod.pending_transaction_with_id(&tx.tx_id).await?;
            if let Some(confirmed_round) = txn.confirmed_round {
                if confirmed_round != 0 {
                    return Ok(txn);
                }
            } else if txn.pool_error != "" {
                return Err(AlgodotError::PoolError(txn.pool_error));
            }
        }
    }
}

#[methods]
impl Algodot {
    #[export]
    fn _enter_tree(&mut self, _owner: TRef<Node>) {
        self.update_algod(_owner);
    }

    fn update_algod(&mut self, _owner: TRef<Node>) {
        // Do not update while in editor
        // e.g. editing properties in the inspector
        if Engine::godot_singleton().is_editor_hint() {
            return;
        }
        let algod: Algod;
        if self.token.is_empty() {
            let headers = self
                .headers
                .read()
                .iter()
                .map(|header| -> Result<(String, String), AlgodotError> {
                    let header = &header.to_string();
                    let mut split = header.split(": ");

                    let get_string = |split: &mut std::str::Split<&str>| {
                        split
                            .next()
                            .map(|str| str.to_string())
                            .ok_or(AlgodotError::HeaderParseError)
                    };

                    Ok((get_string(&mut split)?, get_string(&mut split)?))
                })
                .collect::<Result<Vec<(String, String)>, AlgodotError>>();

            godot_unwrap!(headers => {
                let headers: Vec<(&str, &str)> = headers
                    .iter()
                    .map(|(str1, str2)| -> (&str, &str) { (str1, str2) })
                    .collect();

                algod = AlgodCustomEndpointBuilder::new()
                    .bind(&self.url)
                    .headers(headers)
                    .build_v2()
                    .unwrap();

                self.algod = Rc::new(algod);
            });
        } else {
            algod = AlgodBuilder::new()
                .bind(&self.url)
                .auth(&self.token)
                .build_v2()
                .unwrap();
            self.algod = Rc::new(algod);
        }
    }

    #[export]
    fn generate_key(&self, _owner: TRef<Node>) -> (String, String) {
        let acc = Account::generate();
        (acc.address().to_string(), acc.mnemonic())
    }

    #[export]
    fn sign_transaction(
        &self,
        _owner: TRef<Node>,
        txn: Transaction,
        signer: Account,
    ) -> SignedTransaction {
        SignedTransaction::from(signer.sign_transaction(&txn).unwrap())
    }

    #[export]
    fn construct_payment(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        from: Address,
        to: Address,
        amount: u64,
    ) -> Transaction {
        TxnBuilder::with(
            params.clone(),
            Pay::new(*from, *to, MicroAlgos(amount)).build(),
        )
        .build()
        .into()
    }

    #[export]
    fn construct_asset_xfer(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        asset_id: u64,
        amount: u64,
        receiver: Address,
    ) -> Transaction {
        TxnBuilder::with(
            params.clone(),
            TransactionType::AssetTransferTransaction(AssetTransferTransaction {
                sender: *sender,
                xfer: asset_id,
                amount: amount,
                receiver: *receiver,
                close_to: None,
            }),
        )
        .build()
        .into()
    }

    #[export]
    fn construct_asset_create(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        asset_name: String,
        decimals: u32,
        default_frozen: bool,
        total: u64,
        unit_name: String,
        meta_data_hash: Option<ByteArray>,
        url: Option<String>,
        clawback: Option<Address>,
        freeze: Option<Address>,
        manager: Option<Address>,
        reserve: Option<Address>,
    ) -> Transaction {
        let mdh = meta_data_hash
            .and_then(|mdh| Some(mdh.read().iter().map(|num| *num).collect::<Vec<u8>>()));

        TxnBuilder::with(
            params.clone(),
            TransactionType::AssetConfigurationTransaction(AssetConfigurationTransaction {
                sender: *sender,
                params: Some(AssetParams {
                    asset_name: Some(asset_name),
                    decimals: Some(decimals),
                    default_frozen: Some(default_frozen),
                    total: Some(total),
                    unit_name: Some(unit_name),
                    meta_data_hash: mdh,
                    url: url,
                    clawback: clawback.and_then(|x| Some(*x)),
                    freeze: freeze.and_then(|x| Some(*x)),
                    manager: manager.and_then(|x| Some(*x)),
                    reserve: reserve.and_then(|x| Some(*x)),
                }),
                config_asset: None,
            }),
        )
        .build()
        .into()
    }

    #[export]
    fn construct_app_call(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        app_id: Option<u64>,
        accounts: Option<StringArray>,
        app_arguments: Option<VariantArray>, // array of PoolByteArrays
        foreign_apps: Option<Int32Array>,
        foreign_assets: Option<Int32Array>,
        approval_program: Option<Vec<u8>>,
        clear_state_program: Option<Vec<u8>>,
        global_state_schema: Option<(u64, u64)>,
        local_state_schema: Option<(u64, u64)>,
    ) -> Transaction {
        let accounts = accounts.and_then(|acc| {
            Some(
                acc.read()
                    .iter()
                    .map(|str| *Address::from_variant(&str.to_variant()).unwrap())
                    .collect(),
            )
        });

        let app_arguments: Option<Vec<Vec<u8>>> = app_arguments.and_then(|args| {
            Some(
                args.iter()
                    .map(|var| {
                        var.try_to_byte_array()
                            .unwrap()
                            .read()
                            .iter()
                            .map(|num| *num)
                            .collect::<Vec<u8>>()
                    })
                    .collect(),
            )
        });

        let foreign_apps: Option<Vec<u64>> =
            foreign_apps.and_then(|fa| Some(fa.read().iter().map(|num| *num as u64).collect()));

        let foreign_assets: Option<Vec<u64>> =
            foreign_assets.and_then(|fa| Some(fa.read().iter().map(|num| *num as u64).collect()));

        let approval_program: Option<CompiledTealBytes> =
            approval_program.and_then(|bytes| Some(CompiledTealBytes(bytes)));

        let clear_state_program: Option<CompiledTealBytes> =
            clear_state_program.and_then(|bytes| Some(CompiledTealBytes(bytes)));

        let global_state_schema: Option<StateSchema> =
            global_state_schema.and_then(|(number_ints, number_byteslices)| {
                Some(StateSchema {
                    number_ints,
                    number_byteslices,
                })
            });

        let local_state_schema: Option<StateSchema> =
            local_state_schema.and_then(|(number_ints, number_byteslices)| {
                Some(StateSchema {
                    number_ints,
                    number_byteslices,
                })
            });

        TxnBuilder::with(
            params.clone(),
            TransactionType::ApplicationCallTransaction(ApplicationCallTransaction {
                sender: *sender,
                app_id: app_id,
                on_complete: ApplicationCallOnComplete::NoOp,
                accounts,
                approval_program,
                app_arguments,
                clear_state_program,
                foreign_apps,
                foreign_assets,
                global_state_schema,
                local_state_schema,
                extra_pages: 0,
            }),
        )
        .build()
        .into()
    }

    #[export]
    /// Give transactions same group id
    fn group_transactions(
        &self,
        _owner: TRef<Node>,
        mut txns: Vec<Transaction>,
    ) -> Vec<Transaction> {
        let mut txns_mut_refs: Vec<&mut algonaut::transaction::Transaction> =
            txns.iter_mut().map(|tx| &mut tx.0).collect();
        TxGroup::assign_group_id(txns_mut_refs);
        txns
    }
}

asyncmethods!(algod, node, this,
    fn health(_ctx, _args) -> "health" {
        async move {
            let status = algod.health().await;

            match status {
                Ok(_) => unsafe { node.assume_safe().emit_signal("health", &[0.to_variant()]) }, // OK
                Err(_) => unsafe { node.assume_safe().emit_signal("health", &[1.to_variant()]) }, // FAILED
            };

            ().to_variant()
        }
    };

    fn suggested_transaction_params(_ctx, _args) -> "suggested_transaction_params" {
        async move {
            let params = algod.suggested_transaction_params().await;

            godot_unwrap!(params => {
                let params = to_json_dict(&params);

                unsafe { node.assume_safe().emit_signal("suggested_transaction_params", &[params.to_variant()]) };
            });

            ().to_variant()
        }
    };

    fn status(_ctx, _args) -> "status" {
        async move {
            let status = algod.status().await;

            godot_unwrap!(status => {
                let status = to_json_dict(&status);

                unsafe { node.assume_safe().emit_signal("status", &[status.to_variant()]) };
            });

            ().to_variant()
        }
    };

    fn account_information(_ctx, args) -> "account_info" {
        let address = args.read::<Address>().get().unwrap();

        async move {
            let info = algod.account_information(&address).await;
            godot_unwrap!(info => {
                let info = to_json_dict(&info);

                unsafe { node.assume_safe().emit_signal("account_info", &[info.to_variant()]) };
            });

            ().to_variant()
        }
    };

    fn send_transaction(_ctx, args) -> "transaction_sent" {
        let txn = args.read::<SignedTransaction>().get().unwrap();

        async move {
            let response = algod.broadcast_signed_transaction(&txn).await;

            let send_response = response
                .as_ref()
                .map(|r| to_json_dict(&r));

            godot_unwrap!(send_response => {
                unsafe { node.assume_safe().emit_signal("transaction_sent", &[send_response.to_variant()]) };

                let wait = Algodot::wait_for_transaction(algod, response.unwrap()).await;

                unsafe { node.assume_safe().emit_signal("transaction_confirmed", &[wait.map(|pt| to_json_dict(&pt)).unwrap().to_variant()]) };
            });

            ().to_variant()
        }
    };

    fn send_transactions(_ctx, args) -> "transaction_sent" {
        let vartxns = args.read::<Vec<SignedTransaction>>().get().unwrap();
        let txns: Vec<algonaut::transaction::SignedTransaction> = vartxns.iter().map(|tx| tx.0.clone()).collect();

        async move {
            let response = algod.broadcast_signed_transactions(txns.as_slice()).await;

            let send_response = response
                .as_ref()
                .map(|r| to_json_dict(&r));

            godot_unwrap!(send_response => {
                unsafe { node.assume_safe().emit_signal("transaction_sent", &[send_response.to_variant()]) };

                let wait = Algodot::wait_for_transaction(algod, response.unwrap()).await;

                unsafe { node.assume_safe().emit_signal("transaction_confirmed", &[wait.map(|pt| to_json_dict(&pt)).unwrap().to_variant()]) };
            });

            ().to_variant()
        }
    };

    fn send_algo_transaction(_ctx, args) -> "algo_transaction_sent" {
        let from_account = args.read::<Account>().get().unwrap();
        let to_address = args.read::<Address>().get().unwrap();
        let amount = args.read::<u64>().get().unwrap();

        async move {
            let params = algod.suggested_transaction_params().await.unwrap();

            let t = TxnBuilder::with(
                params,
                Pay::new(from_account.address(), *to_address, MicroAlgos(amount)).build(),
            )
            .build();

            let signed_t = from_account.sign_transaction(&t).unwrap();

            let response = algod.broadcast_signed_transaction(&signed_t).await;
            let send_response = response
                .as_ref()
                .map(|r| to_json_dict(&r));

            godot_unwrap!(send_response => {
                unsafe { node.assume_safe().emit_signal("transaction_sent", &[send_response.to_variant()]) };

                let wait = Algodot::wait_for_transaction(algod, response.unwrap()).await;

                unsafe {
                    node.assume_safe().emit_signal(
                        "transaction_confirmed",
                        &[wait.map(|pt| to_json_dict(&pt)).unwrap().to_variant()]
                    )
                };
            });


            ().to_variant()
        }
    };

    fn compile_teal(_ctx, args) -> "teal_compiled" {
        let source_code = args.read::<String>().get().unwrap();

        async move {
            let compiled = algod.compile_teal(source_code.as_bytes()).await;

            godot_unwrap!(compiled => {
                unsafe {
                    node.assume_safe().emit_signal(
                        "teal_compiled",
                        &[(compiled.hash.0.to_variant(), compiled.program.0.to_variant()).to_variant()]
                    )
                };
            });

            ().to_variant()
        }
    }
);
