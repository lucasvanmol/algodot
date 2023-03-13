#[macro_export]
mod escrow {

    /*Atomic Transaction Composer Helper Modules*/
    use algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner::BasicAccount;
    //use crate::core::Account;
    use algonaut::transaction::account::Account;
    //use crate::algod::Account;
    
    //use algonaut::algod::v2::Algod;
    use algonaut::abi::abi_type::AbiValue::Int;
    use algonaut::core::Address;
    
    //use num_bigint::BigUint;
   // use crate::algod::bar::Foo;
    use algonaut::{
        
        atomic_transaction_composer::{
            transaction_signer::TransactionSigner, AbiArgValue, 
            AtomicTransactionComposer, //AbiReturnDecodeError, AddMethodCallParams, 
            TransactionWithSigner, //AtomicTransactionComposerStatus, 
        },
        error::ServiceError,
    };
    
    use algonaut::core::{to_app_address, Address as OtherAddress, MicroAlgos, CompiledTeal};
    //use algonaut::abi::abi_interactions::AbiMethod;
    use algonaut::transaction::{
        builder::TxnFee, builder::TxnFee::Fixed,
        transaction::{ApplicationCallOnComplete, StateSchema},
        Pay, TxnBuilder,
    };

    use algonaut::core::SuggestedTransactionParams as OtherSuggestedTransactionParams;
    use algonaut::transaction::{transaction::Payment}; //account::Account
  
    //use algonaut::crypto::HashDigest;
    use std::convert::TryInto;   
    //use gdnative::prelude::*;

   // use std::marker::Sized;
    use std::fmt::Display as Display;
    use algonaut::atomic_transaction_composer::AtomicTransactionComposerStatus as OtherAtomicTransactionComposerStatus;

    use crate::algod::bar::Foo as OtherFoo;
    //#[derive(Clone, Debug, escrow::ToVariant::to_variant(&atc))] //PartialEq,
    
    //#[derive(Clone, Debug, escrow::MyTrait::to_variant(&atc))] //PartialEq,
            
    //#[derive(Clone<'_>, Debug<'_>, gdnative::prelude::ToVariant::to_variant(atc))] //PartialEq,
    
    //#[derive(Clone, Debug, escrow::OwnedToVariant::to_variant(&atc))] //PartialEq,

    #[derive(gdnative::prelude::ToVariant::to_variant(&atc), Debug)] //PartialEq,
    //#[derive (gdnative::prelude::ToVariant::to_variant(Foo))]
     

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
    

    //All lifetime traits
    pub trait MyTrait<'a> {
        type Foo ;
        type Params;
        type Parsed;
        type Payment;
        type AtomicTransactionComposer ;
        //type ToVariant;
        type Sized ;//: u64; //= 7u64;
        type ToVariant = dyn ToVariant<Sized = usize>;
        type OwnedToVariant;
        //type NewTrait;
        

        fn _app_id(&self, x: u64) -> u64;
        //fn default() -> Option<String>{ None }
        //fn suggested_tx_params(&self) -> OtherSuggestedTransactionParams { OtherSuggestedTransactionParams::default() }
        fn arg1(withdrw_amt: u64) -> AbiArgValue { AbiArgValue::AbiValue(Int(withdrw_amt.into())) }
        fn arg2(withdrw_amt: u64) -> AbiArgValue { AbiArgValue::AbiValue(Int(withdrw_amt.into())) }

        fn get(&self) -> &'a AtomicTransactionComposer { todo!()}
        
        //fn to_variant(&self) -> dyn NewTrait <Sized = u64>{
        //    (**self).clone().into_shared().to_variant()
        //}
        fn to_variant(&'a self) -> &'a AtomicTransactionComposer {&AtomicTransactionComposer::default()}
  
    }

    pub trait ATC {
        type AtomicTransactionComposer  = algonaut::atomic_transaction_composer::AtomicTransactionComposerStatus;
        type AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus;     
    }

    impl Display for dyn ATC<AtomicTransactionComposer = AtomicTransactionComposer, AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus>{
        fn to_string(&self) -> String;

    }
    //Error Fixer
    impl Display for dyn ATC<AtomicTransactionComposer = algonaut::atomic_transaction_composer::AtomicTransactionComposerStatus, AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus>{

    }
    //code duplicate
    //impl Display for dyn ATC<AtomicTransactionComposerStatus = dyn AtomicTransactionComposerStatus>{
    //    fn to_string(&self) -> String;

    //}
    //trait NewTrait: ToVariant + Sized  {
       
    // fn static_foo<T:NewTrait + ?Sized>(b: &T) {todo!()}
        
    //}

    //Docs: https://godot-rust.github.io/docs/gdnative/prelude/struct.Variant.html

    //trait NewTrait: escrow::ToVariant + Sized {}
    pub trait OwnedToVariant{
        type Sized ;
        
        fn to_variant(&self) -> &AtomicTransactionComposer ;//{ todo!()}
    }

    pub trait ToVariant{
        type Sized ;
        
        fn to_variant(&self) -> &AtomicTransactionComposer ;//{ todo!()}
    }

    pub trait AtomicTransactionComposerStatus{
        fn status(&self) -> dyn AtomicTransactionComposerStatus ;
        fn to_string(&self) -> String;
    }
    /*Implements all traits for Foo Crate*/
    impl <'a, 'c> MyTrait <'a> for OtherFoo{//Foo <'a>{
        type Foo  = OtherFoo;
        type Parsed = Option<String>;
        type Payment = Option<Payment>;
        type Params = Option<OtherSuggestedTransactionParams>;
        type AtomicTransactionComposer =  AtomicTransactionComposer;
        
        type Sized = u32;//: u64; //= 7u64;
        type ToVariant = dyn ToVariant<Sized = usize>;
        type OwnedToVariant = dyn ToVariant<Sized = u32>; // = dyn OwnedToVariant<Sized = usize>;
        fn _app_id(&self, x: u64) -> u64 { x }
        
    }

    //impl Sized for  dyn ToString{
    //
    //
    // }

    impl AtomicTransactionComposerStatus for dyn ToString { 
        fn status(&self ) -> dyn ToString {
            <dyn AtomicTransactionComposerStatus>::to_string(&dyn escrow::AtomicTransactionComposerStatus)
            //"dfadfsdf".to_string()
        }
        fn to_string(&self) -> String{todo!()};
    }
    impl OwnedToVariant for AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer {&Self::default()}
    

    }
    impl ToVariant for AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { &Self::default()}
    
      
    }

    impl ToVariant for &&AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { &AtomicTransactionComposer::default().status().to_string()}
    
      
    }
    impl<'a> ToVariant for &AtomicTransactionComposer {
        type Sized = i32;
        
        fn to_variant(&self) -> &AtomicTransactionComposer { todo!()}
    
      
    }
      
        impl <'a> MyTrait <'_> for &'a AtomicTransactionComposer {
            type Foo = OtherFoo;//Self::Foo;//Foo<'a>;
            type Parsed = Option<String>;
            type Payment= Option<Payment>;
            type Params = Option<OtherSuggestedTransactionParams>;
            type AtomicTransactionComposer = AtomicTransactionComposer;
            
            type Sized = u32;
            type ToVariant= dyn ToVariant<Sized = u32>;
            type OwnedToVariant = dyn ToVariant <Sized = i32>;
            
            //type NewTrait = dyn NewTrait<Sized = i32>;
         fn _app_id(&self, x: u64) -> u64{todo!()}
   
            
            //type ToVariant = dyn ToVariant<Sized = ?Sized>;//T: ?Sized
            //type Sized;
         //fn to_variant(&self) -> &'a AtomicTransactionComposer {todo!()}
  
         //fn to_variant(&self) -> dyn NewTrait {
         //   (**self).clone().into_shared().to_variant()
         //}

         fn to_variant(&self) -> &AtomicTransactionComposer {
            //(**self).len()
            //todo!()\
            &AtomicTransactionComposer::default()
         }
        }


    impl OtherFoo{
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
            
            Temporarily Disabling
            */
 

            todo!()

        }
            
    
        
        //pub fn withdraw(_acct1: Account ){
             /* 
            Withdraw Method Parameters for Escrow SmartContract
            
                Docs: https://docs.rs/num-bigint/0.4.3/num_bigint/struct.BigUint.html

                Does nothing
            */

        //}
        

        //use algonaut_core::Address;
        pub fn pay(to_address : algonaut::core::Address , acct1 : Account, _params : algonaut::core::SuggestedTransactionParams) -> algonaut::transaction::transaction::Transaction{
            /*
                Constructs a Payment Transaction to an Address
            */

             let _t = TxnBuilder::with(

                    &_params,

                    Pay::new(acct1.address(), to_address, MicroAlgos(123_456)).build(),

                )

                .build()
                .unwrap();
            
            return _t;
        }

        pub fn app_address (app_id : &u64) -> Address{
            to_app_address(*app_id)
        }
        
        //pub fn deposit(_algod : Algod , acct1_3 : Account ,  params : algonaut::core::SuggestedTransactionParams) -> algonaut::core::SuggestedTransactionParams {
            /*
            Deposit Method Parameters for Escrow SmartContract
            Unused and Depreciated
           
            Does
            */

        //    let _app_id = 161737986;

            
            //Get Escrow Address From App ID

        //    let _escrow_address = escrow::Foo::app_address(&_app_id); //to_app_address(_app_id.clone());
           
        //    println!(" building Pay transaction to Escrow Address: {}", &_escrow_address);

            //let _t = Foo::pay(_escrow_address, acct1_3.clone().into(), params.clone());                

            // create a transaction with signer with the current transaction

        //    let _signer ;//= TransactionSigner::BasicAccount(acct1_3);


            //let tx_with_signer = TransactionWithSigner { tx: _t, signer: _signer };


        //    let mut atc = AtomicTransactionComposer::default();  

            // Deposit
            // Add Payment Txn to 
            // Should Ideally Match To A Statemachine Behaviour Bloc
            
            //atc.add_transaction(tx_with_signer).unwrap();

        //  params

 
        //}

        pub fn new() -> AtomicTransactionComposer{
        /*
        Constructs a Default Atomic Transation Composer
        */
            AtomicTransactionComposer::default()
        
        }
     
        pub fn address_to_bytes(addr: String) -> [u8; 32]{ 
        /*
        Constructs a 32 Bit Byte Slice froma Given Address String
        */   
            let mut _to_addr: [u8; 32] = [0; 32];
            _to_addr.copy_from_slice(&addr.as_bytes()[..32]);

            _to_addr
            
        }

        //let arg2: AbiArgValue = AbiArgValue::AbiValue(algonaut_abi::abi_type::AbiValue::Address(OtherAddress::new(withdrw_to_addr)));
      
        pub fn address(addr : [u8; 32]) -> AbiArgValue {
            /* Returns an Address abi value from an Address as [u8,32]*/
            AbiArgValue::AbiValue(algonaut::abi::abi_type::AbiValue::Address(OtherAddress::new(addr)))

        } 

        
        pub fn basic_account(mnemonic : &str)  ->  algonaut::atomic_transaction_composer::transaction_signer::TransactionSigner{
            BasicAccount(Account::from_mnemonic(&mnemonic).unwrap())
        
        }

        pub fn fee(amount : u64) -> TxnFee{Fixed(MicroAlgos(amount))}

        //pub fn construct_app_call_method(
        /*
        Constructs an App Call Method as a Rust Module
        
        Depreciated
        */
        
 
        //&self,
        //_app_id: u64,
        //_method: AbiMethod,
        //_method_args: Vec<AbiArgValue>,
        //_fee: TxnFee,//Fixed(MicroAlgos(2500u64)), //make customizable
        //_sender: Address,
        //_on_complete: ApplicationCallOnComplete,
        //_clear_program: Option<CompiledTeal>,
        //_global_schema: Option<StateSchema>,
        //_local_schema: Option<StateSchema>,
        //_extra_pages: u32,
        //_note: Option<Vec<u8>>,
        //_lease: Option<HashDigest>,
        //_rekey_to: Option<Address>,
        //_signer: TransactionSigner,
      //  )
    
        // -> Result<Foo<'_>, ServiceError> {todo!()}
        

    } 

}

