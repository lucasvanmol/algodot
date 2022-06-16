use algonaut::core::{MicroAlgos, Round, SuggestedTransactionParams};
use algonaut::crypto::{HashDigest, Signature};
use algonaut::model::algod::v2::PendingTransaction;
use algonaut::transaction::account::Account;
use algonaut::transaction::transaction::{
    AssetAcceptTransaction, AssetConfigurationTransaction, AssetParams, AssetTransferTransaction,
    Payment, TransactionSignature,
};
use algonaut::transaction::{SignedTransaction, Transaction, TransactionType};
use algonaut::{core::Address, error::ServiceError};
use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut, From, Into};
use gdnative::api::JSON;
use gdnative::prelude::*;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;

// todo: remove unwraps and resturn Result
// check JSONParseResult.error
// https://docs.godotengine.org/en/stable/classes/class_jsonparseresult.html
pub fn to_json_dict<T: Serialize>(r: &T) -> Variant {
    let str = serde_json::to_string(r).unwrap();
    unsafe {
        JSON::godot_singleton()
            .parse(&str)
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

#[derive(Deref, DerefMut)]
pub struct MySuggestedTransactionParams(SuggestedTransactionParams);

impl FromVariant for MySuggestedTransactionParams {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .try_to_dictionary()
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
        to_json_dict(&self.0)
    }
}

// https://developer.algorand.org/docs/get-details/transactions/transactions/#common-fields-header-and-type
impl FromVariant for MyTransaction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .try_to_dictionary()
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
        to_json_dict(&self.0)
    }
}

impl FromVariant for MySignedTransaction {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let dict = variant
            .try_to_dictionary()
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
    Ok(HashDigest({
        let byte_array = get_field(dict, field_name)?;

        let byte_array = byte_array
            .try_to_array()
            .ok_or(FromVariantError::InvalidField {
                field_name,
                error: Box::new(FromVariantError::Custom("must be byte array".to_string())),
            })?;
        if byte_array.len() == 32 {
            let mut slice: [u8; 32] = [0; 32];
            for (i, elem) in byte_array.iter().enumerate() {
                let e = parse_u64(&elem)
                    .and_then(|num| {
                        if num < u8::MAX as u64 {
                            Some(num as u8)
                        } else {
                            None
                        }
                    })
                    .ok_or(FromVariantError::InvalidField {
                        field_name,
                        error: Box::new(FromVariantError::Custom(format!(
                            "element {:?} is not a byte",
                            &elem
                        ))),
                    })?;
                slice[i] = e as u8;
            }
            Ok(slice)
        } else {
            Err(FromVariantError::InvalidField {
                field_name,
                error: Box::new(FromVariantError::Custom(
                    "must be 32 bytes long".to_string(),
                )),
            })
        }
    }?))
}

fn parse_u64(var: &Variant) -> Option<u64> {
    var.try_to_u64().or_else(|| {
        var.try_to_f64().and_then(|num| {
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
        .try_to_bool()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("field must be bool".to_string())),
        })
}

fn get_string(dict: &Dictionary, field_name: &'static str) -> Result<String, FromVariantError> {
    get_field(dict, field_name)?
        .try_to_string()
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("field must be string".to_string())),
        })
}

const HASH_LEN: usize = 32;

fn get_address(dict: &Dictionary, field_name: &'static str) -> Result<Address, FromVariantError> {
    get_field(dict, field_name)?
        .try_to_array()
        .map(|bytes| {
            // x.to_u64() can sometimes return default value, maybe replace with x.try_to_u64()
            let iter: ArrayVec<u8, HASH_LEN> = bytes.iter().map(|x| x.to_u64() as u8).collect();
            let mut slc: [u8; HASH_LEN] = [0; HASH_LEN];
            slc.clone_from_slice(iter.as_slice());
            Address::new(slc)
        })
        .ok_or(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("invalid address".to_string())),
        })
}

fn get_dict(dict: &Dictionary, field_name: &'static str) -> Result<Dictionary, FromVariantError> {
    let var = get_field(dict, field_name)?;

    var.try_to_dictionary()
        .ok_or(FromVariantError::InvalidVariantType {
            variant_type: var.get_type(),
            expected: VariantType::Dictionary,
        })
}

fn get_vec_u8(dict: &Dictionary, field_name: &'static str) -> Result<Vec<u8>, FromVariantError> {
    let var = get_field(dict, field_name)?;
    let byte_array = var
        .try_to_array()
        .map(|bytes| {
            bytes
                .iter()
                .map(|byte| byte.to_u64() as u8)
                .collect::<Vec<u8>>()
        })
        .ok_or(FromVariantError::InvalidVariantType {
            variant_type: var.get_type(),
            expected: VariantType::ByteArray,
        })?;
    Ok(byte_array)
}

// https://developer.algorand.org/docs/get-details/transactions/
fn get_transaction_type(
    dict: &Dictionary,
    field_name: &'static str,
) -> Result<TransactionType, FromVariantError> {
    let txn_type =
        get_field(dict, field_name)?
            .try_to_string()
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
        "appl" => todo!(),
        _ => Err(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("invalid txn type".to_string())),
        }),
    }
}
