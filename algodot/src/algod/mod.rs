<<<<<<< HEAD
=======
//implement Algorand functions here
//use wasm_bindgen::prelude::*; //for building to webassembly

>>>>>>> d2563ad3dd805622d8e5209ef5e2c9393855240f
use algodot_core::*;
use algodot_macros::*;
use algonaut::algod::v2::Algod;
use algonaut::core::{CompiledTeal, MicroAlgos, Round};
use algonaut::model::algod::v2::{PendingTransaction, TransactionResponse};
use algonaut::transaction::transaction::{
    ApplicationCallOnComplete, ApplicationCallTransaction, AssetAcceptTransaction,
    AssetConfigurationTransaction, AssetParams, AssetTransferTransaction, StateSchema,
};
use algonaut::transaction::tx_group::TxGroup;
use algonaut::transaction::{Pay, TransactionType, TxnBuilder};
use gdnative::api::Engine;
use gdnative::prelude::*;
use gdnative::tasks::{Async, AsyncMethod, Spawner};
use std::rc::Rc;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Algodot {
    #[property(set = "Self::set_url")]
    url: String,

    #[property(set = "Self::set_token")]
    token: String,

    #[property(set = "Self::set_headers")]
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
                Algod::new(
                    "http://localhost:4001",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                )
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
        builder
            .signal("transaction_confirmed")
            .with_param_custom(SignalParam {
                name: "transaction_info".into(),
                default: ().to_variant(),
                export_info: ExportInfo::new(VariantType::Dictionary),
                usage: PropertyUsage::DEFAULT,
            })
            .done();
    }

    async fn wait_for_transaction(
        algod: Rc<Algod>,
        tx: TransactionResponse,
    ) -> Result<PendingTransaction, AlgodotError> {
        let status = algod.status().await?;
        let mut round = status.last_round - 1;
        loop {
            algod.status_after_round(Round(round)).await?;
            let txn = algod.pending_transaction_with_id(&tx.tx_id).await?;
            if let Some(confirmed_round) = txn.confirmed_round {
                if confirmed_round != 0 {
                    return Ok(txn);
                }
            } else if !txn.pool_error.is_empty() {
                return Err(AlgodotError::PoolError(txn.pool_error));
            }
            round += 1;
        }
    }
}

#[methods]
impl Algodot {
    #[export]
    fn _enter_tree(&mut self, _owner: TRef<Node>) {
        self.update_algod(_owner);
    }

    #[export]
    fn set_url(&mut self, _owner: TRef<Node>, url: String) {
        self.url = url;
        self.update_algod(_owner);
    }

    #[export]
    fn set_token(&mut self, _owner: TRef<Node>, token: String) {
        self.token = token;
        self.update_algod(_owner);
    }

    #[export]
    fn set_headers(&mut self, _owner: TRef<Node>, headers: StringArray) {
        self.headers = headers;
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

            if let Some(headers) = godot_unwrap!(headers) {
                let headers: Vec<(&str, &str)> = headers
                    .iter()
                    .map(|(str1, str2)| -> (&str, &str) { (str1, str2) })
                    .collect();

                algod = Algod::with_headers(&self.url, headers).unwrap();

                self.algod = Rc::new(algod);
            }
        } else {
            algod = Algod::new(&self.url, &self.token).unwrap();
            self.algod = Rc::new(algod);
        }
    }

    #[export]
    fn generate_key(&self, _owner: TRef<Node>) -> (String, String) {
        let acc = Account::generate();
        (acc.address().to_string(), acc.mnemonic())
    }

    #[export]
    fn get_address(&self, _owner: TRef<Node>, mnemonic: Account) -> Address {
        mnemonic.address().into()
    }

    #[export]
    fn sign_transaction(
        &self,
        _owner: TRef<Node>,
        txn: Transaction,
        signer: Account,
    ) -> Option<SignedTransaction> {
        let stxn = signer.sign_transaction(txn.into());
        godot_unwrap!(stxn).map(SignedTransaction::from)
    }

    #[export]
    fn construct_payment(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        receiver: Address,
        amount: u64,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            Pay::new(*sender, *receiver, MicroAlgos(amount)).build(),
        )
        .build()
        .unwrap()
        .into()
    }

    #[export]
    #[allow(clippy::too_many_arguments)]
    fn construct_asset_xfer(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        receiver: Address,
        amount: u64,
        asset_id: u64,
        #[opt] close_to: Option<Address>,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            TransactionType::AssetTransferTransaction(AssetTransferTransaction {
                sender: *sender,
                xfer: asset_id,
                amount,
                receiver: *receiver,
                close_to: close_to.map(|x| *x),
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    #[export]
    #[allow(clippy::too_many_arguments)]
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
        #[opt] meta_data_hash: Option<ByteArray>,
        #[opt] url: Option<String>,
        #[opt] clawback: Option<Address>,
        #[opt] freeze: Option<Address>,
        #[opt] manager: Option<Address>,
        #[opt] reserve: Option<Address>,
    ) -> Transaction {
        let mdh = meta_data_hash.map(|mdh| mdh.read().iter().copied().collect::<Vec<u8>>());

        TxnBuilder::with(
            &params,
            TransactionType::AssetConfigurationTransaction(AssetConfigurationTransaction {
                sender: *sender,
                params: Some(AssetParams {
                    asset_name: Some(asset_name),
                    decimals: Some(decimals),
                    default_frozen: Some(default_frozen),
                    total: Some(total),
                    unit_name: Some(unit_name),
                    meta_data_hash: mdh,
                    url,
                    clawback: clawback.map(|x| *x),
                    freeze: freeze.map(|x| *x),
                    manager: manager.map(|x| *x),
                    reserve: reserve.map(|x| *x),
                }),
                config_asset: None,
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    #[export]
    #[allow(clippy::too_many_arguments)]
    fn construct_app_call(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        #[opt] app_id: Option<u64>,
        #[opt] accounts: Option<Vec<Address>>,
        #[opt] app_arguments: Option<VariantArray>, // array of PoolByteArrays. Could perhaps be changed directily to Option<Vec<Vec<u8>>>
        #[opt] foreign_apps: Option<Int32Array>,
        #[opt] foreign_assets: Option<Int32Array>,
        #[opt] approval_program: Option<Vec<u8>>,
        #[opt] clear_state_program: Option<Vec<u8>>,
        #[opt] global_state_schema: Option<(u64, u64)>,
        #[opt] local_state_schema: Option<(u64, u64)>,
        #[opt] extra_pages: Option<u32>,
    ) -> Transaction {
        let accounts: Option<Vec<algonaut::core::Address>> =
            accounts.map(|acc| acc.iter().map(|acc| **acc).collect());

        let app_arguments: Option<Vec<Vec<u8>>> = app_arguments.map(|args| {
            args.iter()
                .map(|var| var.to::<Vec<u8>>().unwrap())
                .collect()
        });

        let foreign_apps = foreign_apps.map(|fa| fa.read().iter().map(|num| *num as u64).collect());

        let foreign_assets =
            foreign_assets.map(|fa| fa.read().iter().map(|num| *num as u64).collect());

        let approval_program = approval_program.map(CompiledTeal);

        let clear_state_program = clear_state_program.map(CompiledTeal);

        let global_state_schema =
            global_state_schema.map(|(number_ints, number_byteslices)| StateSchema {
                number_ints,
                number_byteslices,
            });

        let local_state_schema =
            local_state_schema.map(|(number_ints, number_byteslices)| StateSchema {
                number_ints,
                number_byteslices,
            });

        TxnBuilder::with(
            &params,
            TransactionType::ApplicationCallTransaction(ApplicationCallTransaction {
                sender: *sender,
                app_id,
                on_complete: ApplicationCallOnComplete::NoOp,
                accounts,
                approval_program,
                app_arguments,
                clear_state_program,
                foreign_apps,
                foreign_assets,
                global_state_schema,
                local_state_schema,
                extra_pages: extra_pages.unwrap_or(0),
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    #[export]
    fn construct_asset_opt_in(
        &self,
        _owner: TRef<Node>,
        params: SuggestedTransactionParams,
        sender: Address,
        asset_id: u64,
    ) -> Transaction {
        TxnBuilder::with(
            &params,
            TransactionType::AssetAcceptTransaction(AssetAcceptTransaction {
                sender: *sender,
                xfer: asset_id,
            }),
        )
        .build()
        .unwrap()
        .into()
    }

    #[export]
    /// Give transactions same group id
    fn group_transactions(
        &self,
        _owner: TRef<Node>,
        mut txns: Vec<Transaction>,
    ) -> Option<Vec<Transaction>> {
        let mut txns_mut_refs: Vec<&mut algonaut::transaction::Transaction> =
            txns.iter_mut().map(|tx| &mut tx.0).collect();
        let result = TxGroup::assign_group_id(txns_mut_refs.as_mut_slice());
        godot_unwrap!(result).map(|_| txns)
    }
}

asyncmethods!(algod, node, this,
    fn health(_ctx, _args) {
        async move {
            let status = algod.health().await;

            match status {
                Ok(_) => 0.to_variant(), // OK
                Err(_) => 1.to_variant(), // FAILED
            }
        }
    }

    fn suggested_transaction_params(_ctx, _args) {
        async move {
            let params = algod.suggested_transaction_params().await.map(SuggestedTransactionParams::from);
            godot_unwrap!(params).to_variant()
        }
    }

    fn status(_ctx, _args) {
        async move {
            let status = algod.status().await;
            godot_unwrap!(status).map(|status| to_json_dict(&status)).to_variant()
        }
    }

    fn account_information(_ctx, args) {
        let address = args.read::<Address>().get().unwrap();
        async move {
            let info = algod.account_information(&address).await;
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn transaction_information(_ctx, args) {
        let txid = args.read::<String>().get().unwrap();

        async move {
            let info = algod.pending_transaction_with_id(txid.as_ref()).await;
            godot_unwrap!(info).map(|info| to_json_dict(&info)).to_variant()
        }
    }

    fn send_transaction(_ctx, args) {
        let txn = args.read::<SignedTransaction>().get().unwrap();

        async move {
            let txid = algod.broadcast_signed_transaction(&txn).await;
            godot_unwrap!(txid).map(|txid| txid.tx_id).to_variant()
        }
    }

    fn wait_for_transaction(_ctx, args) {
        let tx_id = args.read::<String>().get().unwrap();

        async move {
            let pending_tx = Algodot::wait_for_transaction(algod, TransactionResponse { tx_id }).await;
            godot_unwrap!(pending_tx).map(|tx| to_json_dict(&tx)).to_variant()
        }
    }

    fn send_transactions(_ctx, args) {
        let vartxns = args.read::<Vec<SignedTransaction>>().get().unwrap();
        let txns: Vec<algonaut::transaction::SignedTransaction> = vartxns.iter().map(|tx| tx.0.clone()).collect();

        async move {
            let txid = algod.broadcast_signed_transactions(txns.as_slice()).await;
            godot_unwrap!(txid).map(|txid| to_json_dict(&txid)).to_variant()
        }
    }

    fn compile_teal(_ctx, args) {
        let source_code = args.read::<String>().get().unwrap();

        async move {
            let compiled = algod.compile_teal(source_code.as_bytes()).await;
            godot_unwrap!(compiled).map(|c| (c.hash().0.to_vec().to_variant(), c.bytes_to_sign().to_variant())).to_variant()
        }
    }
);
