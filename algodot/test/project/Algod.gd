# *************************************************
# godot3-Algod Plugin- by Lucasvanmol & INhumanity_arts
# Released under MIT License
# *************************************************
# Algod Script
# Algorand Objects Within the Scene Tree
# *************************************************
# Features
# (1) can compile teal
# (2) can create assets (NFT's) 
# (3) Can create transactions between different accounts
# (4) Can transfer assets between accounts
# (5) Can check account information

# Requires
# (1) An algorand sandbox node for testing and proper debugging

# To Do:
#(1) Make functions easier to read
# (2) Implement function parameters
# (3) Write proper documentation
# (4) Implement signals
#
# *************************************************

extends Node

var algod: Algod


export (String) var funder_mnemonic
export (String) var funder_address
export (String) var url
#var account # for generating new account # depreciated
var params

" For Testing purpose only. Should be encrypted in release build"
export (String) var receivers_mnemonic
export (String) var receivers_address

onready var parent = $parent #for holding algod child node

# placeholder variables
export ( bool) var debug_txn   #debugs my code

export (bool) var generate_new_account = false # generates a new account & Mnemonic for testing
var new_account: Array # new generated account Placeholder
var transferred_assets: bool = false # Asset transfer boolean constructor

" Transaction PlaceHolder Variables"
var tx # single transaction placeholder
var stx #Raw signed transaction placeholder
var txns  # Grouped transaction placeholder
var txid #transaction Id placeholder
var asset_tx # asset transaction placeholder
var asset_index #placeholder for asset index
var optin_tx #placeholder for opt in asset transaction

var _info : Dictionary# account asset info placeholder

var wait # debugs the transaction by waiting until it's completed

func create_algod_node():
	print(" -- Initialize Algod")
	algod = Algod.new() 

	algod.url = "http://localhost:4001" #duplicate of Url variable
	algod.token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
	
	
	# Sorts Node arrangement in the scene tree
	parent.add_child(algod)



func _ready():
	create_algod_node()
	
	# Sorts Node arrangement in the scene tree
	#parent.add_child(algod)


	var status = true

	print(" -- Get funder account")
	#funder_mnemonic = OS.get_environment("ALGODOT_FUNDER_MNEMONIC") #Huh? #This does what # not needed
	if funder_mnemonic == "":
		printerr("   !! Funder's Mnemonic cannot be an empty string. Initialization failed")
		_timeout(1)

	#funder_address = algod.get_address(funder_mnemonic) 

	" These are custom tests for the Script. Run to test that Script works"
	if debug_txn == true: 
		status = status && yield(_test_algod_connection(), "completed") #works
		#status = status && yield(_send_transaction_to_receiver_addr(funder_address , funder_mnemonic , receivers_address , receivers_mnemonic), "completed") #works
		status = status && yield(_send_asset_transfers_to_receivers_address(funder_address , funder_mnemonic , receivers_address , receivers_mnemonic), "completed") #untested
		print (status)

	if status:
		print(" -- Test run completed successfully.")
	else:
		print(" -- Test run completed with errors.")
		OS.exit_code = 1

	print(" -- exiting.")
	get_tree().quit()

func _timeout( wait_time : int):
	yield(get_tree().create_timer(wait_time), "timeout")
	print(" -- Test run is taking too long.")
	OS.exit_code = 1

	print(" -- exiting.")
	get_tree().quit()

func _test_algod_connection(): # original Dev Github Action test
	print(" -- _test_algod_connection")
	
	var status = yield(algod.health(), "completed") == OK
	
	if !status:
		printerr("   !! _test_algod_connection failed")

	return status
	



func _on_Timer_timeout():
	_timeout(7)

' Feed it a Variable to Generate a New account & Mnemonic'
func create_new_account(_account : Array): #it should be fed the account varibles as parameters
	if generate_new_account == true:
		#create new account
		_account = algod.generate_key() 
		print("New Account Details: ",_account[0], '////',_account[1]) # account 0 is account created, accout 1 is mnemonic
		return _account
	elif generate_new_account == false:
		push_error(" Set Generate New Account to True before running this funtion")
		_timeout(1)


"# sends transaction btw two accounts"
func _send_transaction_to_receiver_addr( _funder_address : String, _funder_mnemonic : String, _receivers_address : String , _receivers_mnemonic: String  ): #works #should be fed the receiver and sender's accounts as parameters
	print(" -- _sending_transaction")
	
	print("sending tx")
	
	 
	
	#get suggested parameters
	params = yield(algod.suggested_transaction_params(), "completed")
	

	#create and sign transaction
	tx = algod.construct_payment(params, _funder_address, _receivers_address, 1000000000000000)
	
	#sending the signed transaction
	stx = algod.sign_transaction(tx, _funder_mnemonic)
	
	#generating the transaction ID
	txid = yield(algod.send_transaction(stx), "completed")
	
	#wait for confirmation
	print("waiting for confirmation")
	wait = yield(algod.wait_for_transaction(txid), "completed")
	
	# getting the account infromation
	var info = yield(algod.account_information(_receivers_address), "completed")
	
	#print (info) # fpr debug purposes only
	#verifying the receiver's account's algo holdings
	return info.amount 


" Make Sure the Funder's Address has sufficient Algos or the Code will Break"
func _send_asset_transfers_to_receivers_address(_funder_address : String, _funder_mnemonic : String, _receivers_address : String , _receivers_mnemonic): # 
	print(" -- _sending_asset_transfers")
	
	
	
	
	#you can set parameters fee, but i opted out 
	
	params = yield(algod.suggested_transaction_params(), "completed") #duplicate of :generate_suggested_transaction_parameters()
	
	#creates assets
	create_assets("SamCoin", _receivers_address, 1000, "SC") 
	
	#____________________________
	# Whichever account creates the asset must sign the raw transaction
	#____________________________
	
	
	#debug Tx details
	#print (tx)
	
	#generates Raw signed transaction
	
	stx = algod.sign_transaction(tx, _receivers_mnemonic)
	
	#__________________________________________________________
	#print (stx)#for debug purposes only #works
	#_check_account_information(funder_address, funder_mnemonic) #debugger
	#____________________________________________________________________
	
	
	#Generating transaction Id from signed transaction
	txid = yield(algod.send_transaction(stx), "completed") #breaks and returns null if account doesnt have asset
	
	#print (txid) #for debug purposes only
	#print(new_account[0], '////',new_account[1]) # account 0 is account creator, accout 1 is mnemonic
	
	#wait for transaction to finish sending
	wait= yield(algod.wait_for_transaction(txid), "completed") #returns null if account doesn't have asset
	
	
	
	var tx_info = yield(algod.transaction_information(txid), "completed") #returns null parameters (fixed)
	
	#print (tx_info) #for debug purposes only #returns null value
	
	
	asset_index = int(tx_info.get("asset-index")) # Would return "error non existent int constructor" if the transaction Id fails to generate

	#Opts in to the Asset transaction from the Asset creator's account
	opt_in_asset_transaction(_funder_address, asset_index)
	
	
	
	
	# Signs the Raw transaction
	raw_sign_transactions(optin_tx, _funder_mnemonic)
	#stx = algod.sign_transaction(optin_tx, _funder_mnemonic) duplicate of above line
	#print (stx)
	
	yield(algod.send_transaction(stx), "completed") # sends raw signed transaction to the network



	print("atomic swap")

# constructs a new transaction ; possible for the tx fee
	var algo_tx = algod.construct_payment(
		params,
		_funder_address,
		_receivers_address,
		100
	)

# constructs asset transfer from funder address to receiver address of 1 Aseet
	construct_asset_transfer(_receivers_address, _funder_address, 1, asset_index)


	# Sends grouped transactions
	create_grouped_transaction(algo_tx, asset_tx)
	#var txns = algod.group_transactions([algo_tx, asset_tx]) #ducplicate of code above
	
	# Both accounts sign transaction 
	txns[0] = algod.sign_transaction(txns[0], _funder_mnemonic)
	txns[1] = algod.sign_transaction(txns[1], _receivers_mnemonic)
#----------------------------------------------
 
	#print (txns[0]) #for debug purposes only
	#print (txns[1]) #for debug purposes only
	# send signed transaction
	yield(algod.send_transactions(txns), "completed") 
	

#-------------------------------------------------------------
	# gets account information as a dictionary
	var info = yield(algod.account_information(_receivers_address), "completed") #checks receivers address for asset tranfer #should contain account mnemonic?
	
	#print (asset_index) #for debugging in algod sandbox
	
	var funder_assets = info.assets
	for asset in funder_assets: # Checks users account for certain variables
		if transferred_assets == true:
			#check https://github.com/lucasvanmol/algodot/issues/5#issuecomment-1196307682 for more details about the below conditional
			#if asset["asset-id"] == asset_index && asset["amount"] == 1: #Amount should be the same amount as amount of asset transfered if both accounts are new accounts
			if asset["asset-id"] && asset_index && asset["amount"] != null:
				return true
		else:
			print ("Asset Id :",asset["asset-id"], "//", " Asset Index: ", asset_index,"//", "Asset Amount: ",asset["amount"]) #for debug purposes only
			printerr("   !! _test_asset_transfers failed") #works
			return false

" This function can be expanded upon to print lots of Account specific details"
func _check_account_information(address : String, mnemonic : String, info : String)-> Dictionary: #account debugger #works
	_info = yield(algod.account_information(address,mnemonic), "completed")
	if info == "" or null:
		return (print (_info))
	elif info == "assets" :
		var _a = _info.assets
		return (print (_a))
	elif info == "asset-id":
		var _b = _info.get("asset-id")
		return (print (_b))
	else:
		return 

'Creates a Grouped transaction from 2 individual transactions'
func create_grouped_transaction(txn_1, txn_2):
	txns = algod.group_transactions([txn_1, txn_2])
	return txns

func raw_sign_transactions( transaction, mnemonic : String): # transaction is tx
	stx = algod.sign_transaction(transaction, mnemonic)
	return stx

func create_assets(asset_name : String, to_address : String, Total_supply: int, Unit_name: String): #works # breaks when not using default sandbox creator acct
	print("-----creating asset----", asset_name)
	tx = algod.construct_asset_create( #breaks
		params,
		to_address, # Creator #SDK uses default sandbox wallet and ignores this creator (fixed)
		asset_name,	# Asset name
		2,			# Decimals #i.e how many decimals from the total supply
		false,		# Default frozen?
		Total_supply,		# Total supply # This is 1000.00
		Unit_name		# Unit name eg GTC, TC, GC
	)
	return tx


func construct_asset_transfer( from_address : String, to_address : String, amount_ : int, _asset_index ):
	transferred_assets = true
	asset_tx = algod.construct_asset_xfer( # rewrite this as a separate function
		params,
		from_address,
		to_address,
		amount_,
		_asset_index
	)
	return asset_tx

func generate_suggested_transaction_parameters(): #generates a suggested transaction parameter instead of manually creating one
	params = yield(algod.suggested_transaction_params(), "completed") #it a creates a suggested transaction fee instead of manually inputing one
	return params

func opt_in_asset_transaction( from_address: String, _asset_index):
	print("opt in") # rewrite as separate function
	optin_tx = algod.construct_asset_opt_in(
		params,
		from_address,
		_asset_index
		)
	return optin_tx
# Path is the loaded path to the teal script to be compiled
func compile_teal(path : String): # Teal programs only use Approve()  and Clear() functions
	yield (algod.compile_teal( path), "completed") #compiling teal from pyteal seems more efficient, this is a placeholder code

"Placeholder Functions"
func encrypt():
	print ("Placeholder function")
	pass

func decrypt():
	print ("Placeholder function")
	pass
