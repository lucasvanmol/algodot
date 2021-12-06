use algonaut::core::{MicroAlgos, Round, SuggestedTransactionParams};
use algonaut::crypto::HashDigest;
use algonaut::model::algod::v2::{PendingTransaction, TransactionParams};
use algonaut::transaction::account::Account;
use algonaut::transaction::transaction::{
    AssetConfigurationTransaction, AssetParams, KeyRegistration, Payment, TransactionSignature,
};
use algonaut::transaction::{SignedTransaction, Transaction, TransactionType};
use algonaut::{core::Address, error::AlgonautError};
use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut, From, Into};
use gdnative::api::JSON;
use gdnative::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

// todo: remove unwraps and resturn Result
// check JSONParseResult.error
// https://docs.godotengine.org/en/stable/classes/class_jsonparseresult.html
pub fn to_json_dict<T: Serialize>(r: &T) -> Variant {
    unsafe {
        JSON::godot_singleton()
            .parse(&serde_json::to_string(r).unwrap())
            .unwrap()
            .assume_safe()
            .result()
    }
}

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
    AlgonautError(AlgonautError),
}

impl From<AlgonautError> for AlgodotError {
    fn from(err: AlgonautError) -> Self {
        AlgodotError::AlgonautError(err)
    }
}

#[derive(Deref, DerefMut)]
pub struct MyAddress(Address);

impl FromVariant for MyAddress {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Address::from_str(&variant.to_string())
            .map_err(|err| FromVariantError::Custom(err))
            .map(|addr| MyAddress(addr))
    }
}

#[derive(Deref, DerefMut, From)]
pub struct MyAccount(Account);

impl FromVariant for MyAccount {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        Account::from_mnemonic(&variant.to_string())
            .map_err(|err| FromVariantError::Custom(err.to_string()))
            .map(|addr| MyAccount(addr))
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
            fee: MicroAlgos(get_u64(&dict, "fee")?),
            genesis_hash: get_hash_digest(&dict, "genesis_hash")?,
        };
        Ok(MySuggestedTransactionParams(t))
    }
}

#[derive(Deref, DerefMut, From)]
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
        };

        Ok(MySignedTransaction(st))
    }
}

// Helper functions //

fn get_signature(dict: &Dictionary) -> Result<TransactionSignature, FromVariantError> {
    if let Ok(sig) = get_field(dict, "sig") {
        godot_dbg!(sig);
        todo!()
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
    dict.get(field_name).ok_or(FromVariantError::Custom(format!(
        "Missing field: {0}",
        field_name
    )))
}

fn get_hash_digest(
    dict: &Dictionary,
    field_name: &'static str,
) -> Result<HashDigest, FromVariantError> {
    Ok(HashDigest({
        let byte_array = get_field(&dict, field_name)?;
        dbg!(&byte_array);
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
    var.try_to_u64().or(var.try_to_f64().and_then(|num| {
        if num == (num as u64) as f64 {
            Some(num as u64)
        } else {
            None
        }
    }))
}

fn get_u64(dict: &Dictionary, field_name: &'static str) -> Result<u64, FromVariantError> {
    let var = get_field(&dict, field_name)?;
    parse_u64(&var).ok_or(FromVariantError::InvalidField {
        field_name: field_name,
        error: Box::new(FromVariantError::Custom(
            "must be positive integer".to_string(),
        )),
    })
}

fn get_bool(dict: &Dictionary, field_name: &'static str) -> Result<bool, FromVariantError> {
    get_field(&dict, field_name)?
        .try_to_bool()
        .ok_or(FromVariantError::InvalidField {
            field_name: field_name,
            error: Box::new(FromVariantError::Custom("field must be bool".to_string())),
        })
}

fn get_string(dict: &Dictionary, field_name: &'static str) -> Result<String, FromVariantError> {
    get_field(&dict, field_name)?
        .try_to_string()
        .ok_or(FromVariantError::InvalidField {
            field_name: field_name,
            error: Box::new(FromVariantError::Custom("field must be string".to_string())),
        })
}

const HASH_LEN: usize = 32;

fn get_address(dict: &Dictionary, field_name: &'static str) -> Result<Address, FromVariantError> {
    let ty = get_field(&dict, field_name)?.get_type();
    godot_dbg!(ty);
    get_field(&dict, field_name)?
        .try_to_array()
        .and_then(|bytes| {
            // x.to_u64() can sometimes return default value, maybe replace with x.try_to_u64()
            let iter: ArrayVec<u8, HASH_LEN> = bytes.iter().map(|x| x.to_u64() as u8).collect();
            let mut slc: [u8; HASH_LEN] = [0; HASH_LEN];
            slc.clone_from_slice(iter.as_slice());
            Some(Address::new(slc))
        })
        .ok_or(FromVariantError::InvalidField {
            field_name: field_name,
            error: Box::new(FromVariantError::Custom("invalid address".to_string())),
        })
}

fn get_vec_u8(dict: &Dictionary, field_name: &'static str) -> Result<Vec<u8>, FromVariantError> {
    let byte_array = get_field(&dict, field_name)?.try_to_byte_array().ok_or(
        FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("invalid hash digest".to_string())),
        },
    )?;
    let x = Ok(byte_array.read().iter().map(|num| *num).collect());
    x
}

// https://developer.algorand.org/docs/get-details/transactions/
fn get_transaction_type(
    dict: &Dictionary,
    field_name: &'static str,
) -> Result<TransactionType, FromVariantError> {
    let txn_type =
        get_field(&dict, field_name)?
            .try_to_string()
            .ok_or(FromVariantError::InvalidField {
                field_name,
                error: Box::new(FromVariantError::Custom(
                    "txn field must be a string".to_string(),
                )),
            })?;

    match txn_type.as_str() {
        "pay" => {
            let pay = Payment {
                sender: get_address(&dict, "snd")?,
                receiver: get_address(&dict, "rcv")?,
                amount: MicroAlgos(get_u64(&dict, "amt")?),
                close_remainder_to: get_address(&dict, "close").ok(),
            };
            Ok(TransactionType::Payment(pay))
        }
        "keyreg" => {
            let keyreg = KeyRegistration {
                sender: get_address(&dict, "snd")?,
                vote_pk: todo!(),
                selection_pk: todo!(),
                vote_first: todo!(),
                vote_last: todo!(),
                vote_key_dilution: todo!(),
                nonparticipating: todo!(),
            };
            Ok(TransactionType::KeyRegistration(keyreg))
        }
        "acfg" => {
            let acfg = AssetConfigurationTransaction {
                sender: get_address(&dict, "snd")?,
                params: Some(AssetParams {
                    asset_name: get_string(&dict, "an").ok(),
                    decimals: get_u64(&dict, "dc").ok().and_then(|num| Some(num as u32)),
                    default_frozen: get_bool(&dict, "df").ok(),
                    total: get_u64(&dict, "t").ok(),
                    unit_name: get_string(&dict, "un").ok(),
                    meta_data_hash: get_vec_u8(&dict, "am").ok(),
                    url: get_string(&dict, "au").ok(),
                    clawback: get_address(&dict, "c").ok(),
                    freeze: get_address(&dict, "f").ok(),
                    manager: get_address(&dict, "m").ok(),
                    reserve: get_address(&dict, "r").ok(),
                }),
                config_asset: get_u64(&dict, "caid").ok(),
            };
            Ok(TransactionType::AssetConfigurationTransaction(acfg))
        }
        //"axfer" => Ok(TransactionType::AssetTransferTransaction),
        //"afrz" => Ok(TransactionType::AssetFreezeTransaction),
        //"appl" => Ok(TransactionType::ApplicationTransaction),
        _ => Err(FromVariantError::InvalidField {
            field_name,
            error: Box::new(FromVariantError::Custom("invalid txn type".to_string())),
        }),
    }
}
