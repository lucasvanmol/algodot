//code reference: https://github.com/manuelmauro/algonaut/blob/main/tests/step_defs/integration/abi.rs


  
 
mod bar {
    //use algonaut_abi::abi_interactions::AbiMethodArg;
    //use algonaut_abi::abi_interactions::AbiReturn;
    use algonaut::abi::abi_type::AbiType;


    use algonaut::abi::abi_interactions::AbiMethod;

    pub struct Foo {
        pub name: String,
        pub description: String,
        pub type_: String, 
        pub parsed: Option<String>,
    }

    impl MyTrait for Foo {
        type Foo = Foo;
        type Type = String;
        type Parsed = Option<String>;

        fn new() -> Self::Foo {
            Foo {
                name: "".to_string(),
                description: "".to_string(),
                type_: "".to_string(),
                parsed: None,
            }
        }

        fn r#type() -> String { "".to_string() }
        fn parsed() -> Option<AbiType> { None }
    }

    trait MyTrait {
        type Foo;
        type Type: ToString;
        type Parsed;

        fn new() -> Self::Foo;
        fn r#type() -> String;
        fn parsed() -> Option<AbiType>;
    }

    impl Foo {
        //Doc : https://developer.algorand.org/docs/get-details/transactions/signatures/#single-signatures
        //      https://developer.algorand.org/docs/get-details/dapps/smart-contracts/ABI/?from_query=Method%20Signature#reference-types
        // Boilerplate
        //pub fn new() -> AbiMethod {
        //    let method_sig : String = "withdraw(uint64,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
        //    println!("{}",&method_sig);

        //    AbiMethod::from_signature(&method_sig)
        //    .expect("Error")
            
        //}
        
        pub fn withdraw() -> AbiMethod {
            let method_sig : String = "withdraw(uint64,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
            println!("Method Signature: {}",&method_sig);

            AbiMethod::from_signature(&method_sig)
            .expect("Error")
            
        }

        pub fn deposit() -> AbiMethod {
            let method_sig : String = "deposit(PaymentTransaction,account)void".to_string();
            //let method_sig : String = "add(uint64,uint64)uint128".to_string();

            
            println!("Method Signature: {}",&method_sig);

            AbiMethod::from_signature(&method_sig)
            .expect("Error")
            
        }
   
    }
}

//Custom Params Struct

mod params {


//use rmp_serde::from_slice;
pub mod ATC {
    /*
    Atomic Transaction Composer Required Traits
    */
    use std::string::String as str;

    pub enum AtomicTransactionComposerStatus {
        Building,
        Built,
        Signed,
        Submitted,
        Committed,
    }

    impl std::fmt::Display for AtomicTransactionComposerStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AtomicTransactionComposerStatus::Building => write!(f, "Building"),
                AtomicTransactionComposerStatus::Built => write!(f, "Built"),
                AtomicTransactionComposerStatus::Signed => write!(f, "Signed"),
                AtomicTransactionComposerStatus::Submitted => write!(f, "Submitted"),
                AtomicTransactionComposerStatus::Committed => write!(f, "Committed"),
            }
        }
    }

    pub trait Into {
        type Into;
        type From;
        type T;

        fn into<T: From<Self::T> + ?Sized>(_b: &T) {
            todo!()
        }

        fn into_boxed_str() -> &'static str;
    }

    // Implement the From trait for AtomicTransactionComposerStatus to &'static str
    impl<'a> From<AtomicTransactionComposerStatus> for &'a str {
        fn from(s: AtomicTransactionComposerStatus) -> &'a str {
            Box::leak(Box::new(s.to_string()))
        }
    }

    // Implement the From trait for &mut AtomicTransactionComposerStatus to &str
    impl<'a> From<&'a mut AtomicTransactionComposerStatus> for &'a str {
        fn from(_: &'a mut AtomicTransactionComposerStatus) -> &'a str {
            todo!()
        }
    }
}



pub mod params {
    use algonaut::core::SuggestedTransactionParams;
    //use std::collections::HashMap;
    //use crate::params::from_slice;
    use algonaut::model::algod::v2::TransactionParams;
    use algonaut::core::MicroAlgos;
    use algonaut::core::Round;
    //use algonaut_crypto::HashDigest;
    //use crate::params::SuggestedTransactionParams;

    pub struct MySuggestedTransactionParams(SuggestedTransactionParams);

        
        trait ToVariant {
            type Foo;
            type Params;
            type Parsed;
            type Payment;
            type MyTrait;
            

            fn _app_id(&self, x: u64) -> u64;
            fn default() -> Option<String>{ None }
            //fn parsed() -> Option<AbiType>;

            //fn suggested_tx_params(&self) -> OtherSuggestedTransactionParams { }

            fn to_variant(&self, params : SuggestedTransactionParams) -> TransactionParams { 
                let dict =  algonaut::model::algod::v2::TransactionParams{
                    consensus_version: params.consensus_version,
                    fee_per_byte: MicroAlgos(0u64),
                    genesis_hash: params.genesis_hash,//HashDigest([u8; 32]),
                    genesis_id: params.genesis_id,
                    last_round: Round(0u64),
                    min_fee: MicroAlgos(0u64),
                };

                dict
            }

        }

    }
}


//
pub mod escrow {

    use algonaut::algod::v2::Algod;
    use algonaut::abi::abi_type::AbiValue::Int;
    use algonaut::core::Address;
    
    use num_bigint::BigUint;

    use algonaut::{
        atomic_transaction_composer::{
            transaction_signer::TransactionSigner, AbiArgValue, //AbiMethodReturnValue,
            AtomicTransactionComposer, //AbiReturnDecodeError, AddMethodCallParams, 
            TransactionWithSigner, //AtomicTransactionComposerStatus, 
        },
        error::ServiceError,
    };
    use algonaut::abi::{
        abi_interactions::{AbiMethod}, //AbiArgType,AbiReturn,, ReferenceArgType,, AbiReturnType  
        //abi_type::{AbiType, AbiValue as OtherAbiValue},
    };
    use algonaut::core::{to_app_address, Address as OtherAddress, MicroAlgos, CompiledTeal, SuggestedTransactionParams};
    
    use algonaut::transaction::{
        builder::TxnFee, builder::TxnFee::Fixed,
        transaction::{ApplicationCallOnComplete, StateSchema},
        Pay, TxnBuilder,
    };

    use algonaut::core::SuggestedTransactionParams as OtherSuggestedTransactionParams;
    use algonaut::transaction::{transaction::Payment, account::Account};
  
    use algonaut::crypto::HashDigest;
  
    use std::convert::TryInto;
   
    //use crate::params::params::MySuggestedTransactionParams;
    
    use algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner::BasicAccount;
    
    #[derive(Debug)]
    //lifetime Parameter
    pub struct Foo <'a> {
        pub withdrw_amt: u32,
        pub withdrw_to_addr: [u8; 32],
        pub arg1: AbiArgValue,
        pub arg2: AbiArgValue,
        pub _app_id: u64,
        pub _escrow_address: Address,
        pub atc: &'a AtomicTransactionComposer,
    }

    trait MyTrait {
        type Foo <'a>;
        type Params;
        type Parsed;
        type Payment;

        fn _app_id(&self, x: u64) -> u64;
        //fn default() -> Option<String>{ None }
        //fn suggested_tx_params(&self) -> OtherSuggestedTransactionParams { OtherSuggestedTransactionParams::default() }
        fn arg1(withdrw_amt: u64) -> AbiArgValue{AbiArgValue::AbiValue( Int(withdrw_amt.into()))}
        fn arg2(withdrw_amt: u64) -> AbiArgValue{AbiArgValue::AbiValue( Int(withdrw_amt.into()))}
    }

    impl MyTrait for Foo <'_>{
        type Foo <'a> = Foo<'a>;
        type Parsed = Option<String>;
        type Payment = Option<Payment>;
        type Params = Option<OtherSuggestedTransactionParams>;
        fn _app_id(&self, x: u64) -> u64 { x }
    }

    impl Foo <'_> {
        // Adding method to create application call
        fn get_call(&self) -> Result<ApplicationCallOnComplete, ServiceError> {
            //let func_args = vec![self.arg1.clone(), self.arg2.clone()];
            
            todo!()
            
        }

        // Adding method to create pay transaction
        fn get_payment(&self) -> Result<Payment, ServiceError> {
            todo!()
           // tx
        }

        fn arg1(&self)-> AbiArgValue{ 
            todo!()
            
        }
        
        pub fn note(size : u32) -> Option <Vec<u8>>{
            Some(vec![size.try_into().unwrap()])

        }
    


  

        pub fn withdraw_amount(amount : u32) -> AbiArgValue {
            /*
            Converts a U64 int to Big Uint and returns an AbiArg Value
    
            //code reference: https://github.com/manuelmauro/algonaut/blob/main/tests/step_defs/integration/abi.rs

            */
            //let withdrw_amt : num_bigint::BigUint = BigUint::new(vec![amount]); //in MicroAlgos

            let arg1 : AbiArgValue = AbiArgValue::AbiValue( Int(amount.into()));
            arg1
            

        }

    }

}








    

            

            
    
   