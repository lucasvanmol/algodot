# Algodot Node

An instance of the Algorand blockchain's node in Godot engine written in rust.

# Methods

(1) _enter_tree()
    called when the Algod node enters the scene tree

(2) account_information(String: address, String: Mnemonic) 
    gets account information  from a given address. Requires 
    a running algorand sandbox node. Needs the wallet's address
    and sometimes the mnemonic.

(3) compile_teal(String: "res//path_to_script")
    compiles teal 

(4) construct_app_call()

(5) construct_asset_create()
    creates algorand assets

(6) construct_asset_xfer()
    creates an asset transfer tx between two wallets

(7) construct_payment()
    constructs a payment transaction between two or more wallets. 
    can also construct a group signed transaction

(8) generate_keys()
    generates a new account with mnemonic stored to a dictionary

(10) get_address()

(11) group_transactions()
     constructs a group tx that can be signed by both wallet addresses

(12) health()
     checks the health of connection of the plugin to the algorand node

(13) headers 
     a rust parameter

(14) send_transactions()
     sends a tx between a receiver and funder wallet

(15) send_transactions()
     sends a grouped tx between wallets

(16) sets_headers()
     a rust related function

(17) set_url()
     sets the algod node url 

(18) sign_transaction()
     signs a transaction between two wallets

(19) status()
     checks the status of a transaction

(20) suggested_transaction_params()
     uses the Algorand suggested transaction parameters. The alternative is to 
     manually set the parameters, including the transaction fee, and other tx 
     parameters

(21) token()

(22) transaction_information()
     gets the transaction information from a transaction id

(23) Url (path)
     A path to the url that te algorand node uses

(24) wait_for_transaction( tx_id )
     waits for a transaction to finish processing, usually 4-5 seconds. 
     it takes the transaction id as a parameter