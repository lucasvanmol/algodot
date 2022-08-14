# Algodot GDscript Functions

(1) create_algot_node()
    creates an algod node, tokens, url and other parameters can be specified.
    It must be run before any transaction logic is done, else the script would
    return a null Algod variable as an error.

(2) _timeout()
    quits the entire scene tree once the wait time runs out

(3) _send_transaction_to_receiver_addr()
    sends a transaction to from a funders's address to a receiver's address

(4) _send_asset_transfer_to_receivers_address()
    transfers assets from a sender's address to a receiver's address

(5) _check_account_information()
    checks acount information using a given address and mnemonic and returns a 
    dictionary of account details

(6) create_new_account()
    creates a new account, generates a mnemonic for that account and stores it to 
    an array

(7) create_group_signed_transaction()
    creates a raw signed transaction and returns a variable txns

(8) raw_sign_transaction()
    creates a raw signed transaction and returns it as a stx variable

(9) create_assets()
    creates assets "ASA" to a given algorand account

(10) construct_asset_transfer()
     constructs an asset transfer of a fixed amount to other wallets, from a given wallet

(11) generate_suggested_transaction_parameters()
     It generates a suggested transaction parameter and returns a variable params.
     It also creates a suggested transaction fee instead of manually inputing one

(12) opt_in_asset_transaction()
     wallets opt in to a transaction of an asset with an asset ID and returns a 
     variable optin_tx

(13) compile_teal()
     compiles a teal script from a given path

(14) encrypt()
     encrypts data sent to the function

(15) decrypt()
     decrypts data sent to the function