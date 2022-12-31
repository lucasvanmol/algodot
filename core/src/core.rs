//Interracts with the godot debugger. Handles transaction types and prints out Algod node errors
// It uses Transaction Types to trigger a state machine in Algodot core.rs

use algonaut::core::{MicroAlgos, Round, SuggestedTransactionParams};
use algonaut::crypto::{HashDigest, Signature};
use algonaut::model::algod::v2::PendingTransaction;
use algonaut::transaction::account::Account;
use algonaut::transaction::transaction::{
    ApplicationCallOnComplete, ApplicationCallTransaction, AssetAcceptTransaction,
    AssetConfigurationTransaction, AssetParams, AssetTransferTransaction, Payment,
    TransactionSignature,
};

use algonaut::transaction::{SignedTransaction, Transaction, TransactionType};
use algonaut::{core::Address, error::ServiceError};
use derive_more::{Deref, DerefMut, From, Into};
use gdnative::api::JSON;
use gdnative::prelude::*;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

/// This file contains implementations of ToVariant and FromVariant for algonaut types.
///
/// It might be worth looking into forking algonaut instead, allowing the use of `#[derive]` directly.
/// It would be cool to be able to leverage algonaut's `serde` implementations somehow (essentially
/// using `serde_json` to get a Godot Dictionary, but with the right types.

// TODO: remove to_json_dict. Implement ToVariant for types instead
//#[deprecated]
pub fn to_json_dict<T: Serialize>(r: &T) -> Variant {
    let str = serde_json::to_string(r).unwrap();
    unsafe {
        JSON::godot_singleton()
            .parse(str)
            .unwrap()
            .assume_safe()
            .result()
    }
}

#[allow(dead_code)]
pub fn to_json_string<T: OwnedToVariant + ToVariant>(r: &T) -> GodotString {
    JSON::godot_singleton().print(r, "", false)
}

#[derive(Error, Debug)]
pub enum AlgodotError {
    #[error("error parsing header")]
    HeaderParseError,
    #[error("pool error: `{0}`")]
    PoolError(String),
    #[error("algonaut error:`{0}`")]
    ServiceError(ServiceError),
}

impl From<ServiceError> for AlgodotError {
    fn from(err: ServiceError) -> Self {
        AlgodotError::ServiceError(err)
    }
}

#[derive(Debug, Deref, DerefMut, From)]
pub struct MyAddress(Address);

impl FromVariant for MyAddress {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Address::from_str(&variant.to_string())
            .map_err(FromVariantError::Custom)
            .map(MyAddress)
    }
}

impl ToVariant for MyAddress {
    fn to_variant(&self) -> Variant {
        (*self).to_string().to_variant()
    }
}

#[derive(Deref, DerefMut, From)]
pub struct MyAccount(Account);

impl FromVariant for MyAccount {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Account::from_mnemonic(&variant.to_string())
            .map_err(|err| FromVariantError::Custom(err.to_string()))
            .map(MyAccount)
    }
}

impl MyAccount {
    pub fn generate() -> MyAccount {
        Account::generate().into()
    }
}

#[derive(Deref, DerefMut, From)]
pub struct MySuggestedTransactionParams(SuggestedTransactionParams);

//used when constructing To a variant
impl ToVariant for MySuggestedTransactionParams {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("genesis_id", &self.genesis_id);
        dict.insert("first_valid", self.first_valid.0);
        dict.insert("last_valid", self.last_valid.0);
        dict.insert("consensus_version", &self.consensus_version);
        dict.insert("min_fee", self.min_fee.0);
        dict.insert("fee_per_byte", self.fee_per_byte.0);
        dict.insert("genesis_hash", ByteArray::from_slice(&self.genesis_hash.0));
        dict.owned_to_variant()
    }
}

impl FromVariant for MySuggestedTransactionParams {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .to::<Dictionary>()
            .ok_or(FromVariantError::InvalidVariantType {
                variant_type: variant.get_type(),
                expected: VariantType::Dictionary,
            })?;

        let t = SuggestedTransactionParams {
            genesis_id: get_string(&dict, "genesis_id")?,
            first_valid: Round(get_u64(&dict, "first_valid")?),
            last_valid: Round(get_u64(&dict, "last_valid")?),
            consensus_version: get_string(&dict, "consensus_version")?,
            min_fee: MicroAlgos(get_u64(&dict, "min_fee")?),
            fee_per_byte: MicroAlgos(get_u64(&dict, "fee_per_byte")?),
            genesis_hash: get_hash_digest(&dict, "genesis_hash")?,
        };
        Ok(MySuggestedTransactionParams(t))
    }
}

#[derive(Deref, DerefMut, From, Into)]
pub struct MyTransaction(pub Transaction);

impl ToVariant for MyTransaction {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("fee", self.fee.0);
        dict.insert("fv", self.first_valid.0);
        dict.insert("gh", ByteArray::from_slice(&self.genesis_hash.0));
        dict.insert("lv", self.last_valid.0);
        dict.insert(
            "type",
            match &self.txn_type {
                //state machine prints to debug log : https://docs.rs/algonaut_transaction/0.4.2/algonaut_transaction/transaction/enum.TransactionType.html
                TransactionType::Payment(payment) => {
                    dict.insert("snd", MyAddress::from(payment.sender));
                    dict.insert("rcv", MyAddress::from(payment.receiver));
                    dict.insert("amt", payment.amount.0);
                    if let Some(close) = payment.close_remainder_to {
                        dict.insert("close", MyAddress::from(close))
                    };
                    "pay"
                }
                TransactionType::KeyRegistration(_) => todo!(),
                TransactionType::AssetConfigurationTransaction(cfg) => {
                    dict.insert("snd", MyAddress(cfg.sender));
                    let apar = Dictionary::new();
                    if let Some(config_asset) = cfg.config_asset {
                        apar.insert("caid", config_asset);
                    }
                    if let Some(params) = &cfg.params {
                        if let Some(asset_name) = &params.asset_name {
                            apar.insert("an", asset_name)
                        }
                        if let Some(decimals) = &params.decimals {
                            apar.insert("dc", decimals)
                        }
                        if let Some(default_frozen) = &params.default_frozen {
                            apar.insert("df", default_frozen)
                        }
                        if let Some(total) = &params.total {
                            apar.insert("t", total)
                        }
                        if let Some(unit_name) = &params.unit_name {
                            apar.insert("un", unit_name)
                        }
                        if let Some(meta_data_hash) = &params.meta_data_hash {
                            apar.insert("am", meta_data_hash)
                        }
                        if let Some(url) = &params.url {
                            apar.insert("au", url)
                        }
                        if let Some(clawback) = &params.clawback {
                            apar.insert("c", MyAddress::from(*clawback))
                        }
                        if let Some(freeze) = &params.freeze {
                            apar.insert("f", MyAddress::from(*freeze))
                        }
                        if let Some(manager) = &params.manager {
                            apar.insert("m", MyAddress::from(*manager))
                        }
                        if let Some(reserve) = &params.reserve {
                            apar.insert("r", MyAddress::from(*reserve))
                        }
                    }
                    dict.insert("apar", apar);
                    "acfg"
               
                }             
                //https://docs.rs/algonaut_transaction/0.4.2/algonaut_transaction/transaction/struct.AssetTransferTransaction.html
                TransactionType::AssetTransferTransaction(axfer) => {
                    dict.insert("snd", MyAddress::from(axfer.sender));
                    dict.insert("xaid", axfer.xfer);
                    dict.insert("aamt", axfer.amount);
                    dict.insert("arcv", MyAddress::from(axfer.receiver));
                    if let Some(close_to) = axfer.close_to {
                        dict.insert("aclose", MyAddress::from(close_to));
                    }
                    
 
                    "axfer"             
                }
                TransactionType::AssetAcceptTransaction(axfer) => {
                    dict.insert("snd", MyAddress::from(axfer.sender));
                    dict.insert("xaid", axfer.xfer);
                    "axfer"
                
                }             
                ///https://docs.rs/algonaut_transaction/0.4.2/algonaut_transaction/transaction/struct.ApplicationCallTransaction.html
                ///defaults to a noOp on transaction complete
                ///should be further customized to include ClearState,CloseOut,DeleteApplication
                TransactionType::ApplicationCallTransaction(appl) => {
                    //Creates a Txn Dictionary for Signing the App Call Txn

                    //creates a Byte Array from app_arg
                    let q: ByteArray = get_byte_array(appl.app_arguments.as_ref().unwrap().clone())
                        .unwrap_or_default();        
                    dict.insert("app_id", appl.app_id);
                    dict.insert("app_arg", q);
                    dict.insert("txn", Dictionary::new());
                    dict.insert("snd", MyAddress::from(appl.sender));
                    "appl"
                }
                TransactionType::AssetClawbackTransaction(_) => todo!(),
                TransactionType::AssetFreezeTransaction(_) => todo!(),
            },
        );
        if let Some(gen) = &self.genesis_id {
            dict.insert("gen", gen);
        }
        if let Some(grp) = &self.group {
            dict.insert("grp", ByteArray::from_slice(&grp.0));
        }
        if let Some(lx) = &self.lease {
            dict.insert("lx", ByteArray::from_slice(&lx.0));
        }
        if let Some(note) = &self.note {
            dict.insert("note", ByteArray::from_slice(note.as_slice()));
        }
        if let Some(rekey) = self.rekey_to {
            dict.insert("rekey", MyAddress::from(rekey));
        }
        dict.owned_to_variant()
    }
}

// https://developer.algorand.org/docs/get-details/transactions/transactions/#common-fields-header-and-type
impl FromVariant for MyTransaction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .to::<Dictionary>()
            .ok_or(FromVariantError::InvalidVariantType {
                variant_type: variant.get_type(),
                expected: VariantType::Dictionary,
            })?;
        let t = Transaction {
            fee: MicroAlgos(get_u64(&dict, "fee")?),
            first_valid: Round(get_u64(&dict, "fv")?),
            genesis_hash: get_hash_digest(&dict, "gh")?,
            last_valid: Round(get_u64(&dict, "lv")?),
            txn_type: get_transaction_type(&dict, "type")?,
            genesis_id: get_string(&dict, "gen").ok(),
            group: get_hash_digest(&dict, "grp").ok(),
            lease: get_hash_digest(&dict, "lx").ok(),
            note: get_vec_u8(&dict, "note").ok(),
            rekey_to: get_address(&dict, "rekey").ok(),
        };
        Ok(MyTransaction(t))
    }
}

#[derive(Deref, DerefMut, From)]
pub struct MyPendingTransaction(pub PendingTransaction);

impl ToVariant for MyPendingTransaction {
    fn to_variant(&self) -> Variant {
        to_json_dict(&self.0)
    }
}

#[derive(Deref, DerefMut, From, Debug)]
pub struct MySignedTransaction(pub SignedTransaction);

impl ToVariant for MySignedTransaction {
    fn to_variant(&self) -> Variant {
        let dict = Dictionary::new();
        dict.insert("txn", MyTransaction::from(self.transaction.clone()));
        match self.sig {
            TransactionSignature::Single(sig) => dict.insert("sig", ByteArray::from_slice(&sig.0)),
            TransactionSignature::Multi(_) => todo!(),
            TransactionSignature::Logic(_) => todo!(),
        }
        //dict.insert("txid", self.transaction_id.to_variant());
        if let Some(sgnr) = self.auth_address {
            dict.insert("sgnr", MyAddress::from(sgnr));
        }
        dict.owned_to_variant()
    }
}

impl FromVariant for MySignedTransaction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .to::<Dictionary>()
            .ok_or(FromVariantError::InvalidVariantType {
                variant_type: variant.get_type(),
                expected: VariantType::Dictionary,
            })?;

        let txn = MyTransaction::from_variant(&get_field(&dict, "txn")?)?;
        let id = txn.id().unwrap();

        let st = SignedTransaction {
            transaction: txn.0,
            transaction_id: id,
            sig: get_signature(&dict)?,
            auth_address: None,
        };

        Ok(MySignedTransaction(st))
    }
}

///Convert's appl call txn to Godot Variants
///#[derive(Deref, DerefMut, From, Debug)]
//pub struct MyApplCallTransaction(pub ApplicationCallTransaction);
///
///impl FromVariant for MyApplCallTransaction {
///    fn from_variant(&self) -> Variant {
///     to_json_dict(&self)
///   }
///
///}    

// Helper functions //

const SIG_LEN: usize = 64;
fn get_signature(dict: &Dictionary) -> Result<TransactionSignature, FromVariantError> {
    if let Ok(_sig) = get_field(dict, "sig") {
        let bytes = get_vec_u8(dict, "sig")?;

        if bytes.len() != SIG_LEN {
            Err(FromVariantError::InvalidLength {
                len: bytes.len(),
                expected: SIG_LEN,
            })
        } else {
            let mut signature = [0u8; SIG_LEN];

            signature.clone_from_slice(&bytes);

            Ok(TransactionSignature::Single(Signature(signature)))
        }
    } else if let Ok(msig) = get_field(dict, "msig") {
        godot_dbg!(msig);
        todo!()
    } else if let Ok(lsig) = get_field(dict, "lsig") {
        godot_dbg!(lsig);
        todo!()
    } else {
        Err(FromVariantError::Custom(
            "Missing signature field".to_string(),
        ))
    }
}

fn get_field(dict: &Dictionary, field_name: &str) -> Result<Variant, FromVariantError> {
    dict.get(field_name)
        .ok_or_else(|| FromVariantError::Custom(format!("Missing field: {0}", field_name)))
}

fn get_hash_digest(
    dict: &Dictionary,
    field_name: &'static str,
) -> Result<HashDigest, FromVariantError> {
    let byte_array = get_field(dict, field_name)?;

    let byte_array = byte_array
        .to::<ByteArray>()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("must be byte array".to_string())),
        })?;
    let mut slice: [u8; 32] = [0; 32];
    if byte_array.len() == 32 {
        slice.copy_from_slice(byte_array.to_vec().as_slice());
        Ok(HashDigest(slice))
    } else {
        Err(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom(
                "must be 32 bytes long".to_string(),
            )),
        })
    }
}

fn parse_u64(var: &Variant) -> Option<u64> {
    var.to::<u64>().or_else(|| {
        var.to::<f64>().and_then(|num| {
            if num == (num as u64) as f64 {
                Some(num as u64)
            } else {
                None
            }
        })
    })
}

fn get_u64(dict: &Dictionary, field_name: &'static str) -> Result<u64, FromVariantError> {
    let var = get_field(dict, field_name)?;
    parse_u64(&var).ok_or(FromVariantError::InvalidField {
        field_name,
        error: Box::new(FromVariantError::Custom(
            "must be positive integer".to_string(),
        )),
    })
}

fn get_bool(dict: &Dictionary, field_name: &'static str) -> Result<bool, FromVariantError> {
    get_field(dict, field_name)?
        .to::<bool>()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("field must be bool".to_string())),
        })
}

fn get_string(dict: &Dictionary, field_name: &'static str) -> Result<String, FromVariantError> {
    get_field(dict, field_name)?
        .to::<String>()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("field must be string".to_string())),
        })
}

fn get_address(dict: &Dictionary, field_name: &'static str) -> Result<Address, FromVariantError> {
    let addr_string =
        get_field(dict, field_name)?
            .to::<String>()
            .ok_or(FromVariantError::InvalidField {
                field_name,
                error: Box::new(FromVariantError::Custom(
                    "address must be a string".to_string(),
                )),
            })?;

    let address =
        Address::from_str(addr_string.as_str()).map_err(|e| FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom(format!(
                "error parsing address: {e}"
            ))),
        })?;
    Ok(address)
    //const HASH_LEN: usize = 32;

    // let bytes = get_vec_u8(dict, field_name)?;

    // let mut slice: [u8; HASH_LEN] = [0; HASH_LEN];
    // if bytes.len() == HASH_LEN {
    //     slice.copy_from_slice(bytes.as_slice());
    //     Ok(Address::new(slice))
    // } else {
    //     Err(FromVariantError::InvalidField {
    //         field_name,
    //         error: Box::new(FromVariantError::Custom(format!(
    //             "address must be {HASH_LEN} byte array"
    //         ))),
    //     })
    // }
}

fn get_dict(dict: &Dictionary, field_name: &'static str) -> Result<Dictionary, FromVariantError> {
    let var = get_field(dict, field_name)?;

    var.to::<Dictionary>()
        .ok_or(FromVariantError::InvalidVariantType {
            variant_type: var.get_type(),
            expected: VariantType::Dictionary,
        })
}

fn get_vec_u8(dict: &Dictionary, field_name: &'static str) -> Result<Vec<u8>, FromVariantError> {
    let var = get_field(dict, field_name)?;
    let byte_array = var
        .to::<ByteArray>()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::InvalidVariantType {
                variant_type: var.get_type(),
                expected: VariantType::ByteArray,
            }),
        })?
        .to_vec();
    Ok(byte_array)
}

//converts a <Vec<Vec<u8>>> to u8
#[allow(dead_code)]
fn get_byte_array(vector: Vec<Vec<u8>>) -> Result<ByteArray, FromVariantError> {
    let byte_array = ByteArray::from_vec(vector.into_iter().next().unwrap_or_default());
    Ok(byte_array)
}

// https://developer.algorand.org/docs/get-details/transactions/
fn get_transaction_type(
    dict: &Dictionary,
    field_name: &'static str,
) -> Result<TransactionType, FromVariantError> {
    let txn_type =
        get_field(dict, field_name)?
            .to::<String>()
            .ok_or(FromVariantError::InvalidField {
                field_name,
                error: Box::new(FromVariantError::Custom(
                    "txn field must be a string".to_string(),
                )),
            })?;

    #[allow(clippy::diverging_sub_expression)] // todo!()
    match txn_type.as_str() {
        "pay" => {
            let pay = Payment {
                sender: get_address(dict, "snd")?,
                receiver: get_address(dict, "rcv")?,
                amount: MicroAlgos(get_u64(dict, "amt")?),
                close_remainder_to: get_address(dict, "close").ok(),
            };
            Ok(TransactionType::Payment(pay))
        }
        "acfg" => {
            let params = get_dict(dict, "apar")?;

            let acfg = AssetConfigurationTransaction {
                sender: get_address(dict, "snd")?,
                params: Some(AssetParams {
                    asset_name: get_string(&params, "an").ok(),
                    decimals: get_u64(&params, "dc").ok().map(|num| num as u32),
                    default_frozen: get_bool(&params, "df").ok(),
                    total: get_u64(&params, "t").ok(),
                    unit_name: get_string(&params, "un").ok(),
                    meta_data_hash: get_vec_u8(&params, "am").ok(),
                    url: get_string(&params, "au").ok(),
                    clawback: get_address(&params, "c").ok(),
                    freeze: get_address(&params, "f").ok(),
                    manager: get_address(&params, "m").ok(),
                    reserve: get_address(&params, "r").ok(),
                }),
                config_asset: get_u64(dict, "caid").ok(),
            };
            Ok(TransactionType::AssetConfigurationTransaction(acfg))
        }
        "keyreg" => todo!(),
        "axfer" => {
            if let Ok(amount) = get_u64(dict, "aamt") {
                let axfer = AssetTransferTransaction {
                    sender: get_address(dict, "snd")?,
                    xfer: get_u64(dict, "xaid")?,
                    amount,
                    receiver: get_address(dict, "arcv")?,
                    close_to: get_address(dict, "aclose").ok(),
                };
                Ok(TransactionType::AssetTransferTransaction(axfer))
            } else {
                let axfer = AssetAcceptTransaction {
                    sender: get_address(dict, "snd")?,
                    xfer: get_u64(dict, "xaid")?,
                };
                Ok(TransactionType::AssetAcceptTransaction(axfer))
            }
        }
        "afrz" => todo!(),       
        "appl" => {
            //checks that the app call is valid
            let appl = ApplicationCallTransaction {
                sender: get_address(dict, "snd")?,
                app_id: Some(get_u64(dict, "app_id")?),
                on_complete: ApplicationCallOnComplete::NoOp,
                accounts: None,
                approval_program: None,
                app_arguments: Some(vec![get_vec_u8(dict, "app_arg")?]),
                clear_state_program: None,
                foreign_apps: None,
                foreign_assets: None,
                global_state_schema: None,
                local_state_schema: None,
                extra_pages: 0u32,
            };
            Ok(TransactionType::ApplicationCallTransaction(appl))
        }

        _ => Err(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("invalid txn type".to_string())),
        }),
    }
}
