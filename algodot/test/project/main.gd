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
var account # for generating new account
var params

" For Testing purpose only. Should be encrypted in release build"
export (String) var receivers_mnemonic
export (String) var receivers_address

onready var parent = $parent #for holding algod child node

# placeholder variables
var debug : bool = false #turns off debug array parser
var debug_txn : bool = true #debugs my code

var tx #transaction placeholder
var stx #Raw signed transaction placeholder
var txid #transaction Id placeholder

var _info : Dictionary# account asset info placeholder

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
	parent.add_child(algod)


	var status = true

	print(" -- Get funder account")
	#funder_mnemonic = OS.get_environment("ALGODOT_FUNDER_MNEMONIC") #Huh? #This does what # not needed
	if funder_mnemonic == "":
		printerr("   !! Funder's Mnemonic cannot be an empty string. Initialization failed")
		#funder_mnemonic = "letter nasty produce hidden confirm sad color diamond allow ring truth code mirror atom obscure this opinion one life travel chat lobster cook about flight"
		_timeout(1)

	funder_address = algod.get_address(funder_mnemonic) 

	if debug == true: # Runs the original dev's default tests
		status = status && yield(_test_algod_connection(), "completed") #works
		status = status && yield(_test_transaction(), "completed") #works
		status = status && yield(_test_asset_transfers(), "completed") #breaks


	if debug_txn == true: # runs my custom debug tests
		status = status && yield(_test_algod_connection(), "completed") #works
		status = status && yield(_test_transaction_to_receiver_addr(), "completed") #works
		status = status && yield(_test_asset_transfers_to_receivers_address(), "completed") #breaks


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

func _test_algod_connection():
	print(" -- _test_algod_connection")
	
	var status = yield(algod.health(), "completed") == OK
	
	if !status:
		printerr("   !! _test_algod_connection failed")

	return status
	
func _test_transaction(): # try this with already created account
	print(" -- _test_transaction")
	
	print("sending tx")
	#get suggested parameters
	params = yield(algod.suggested_transaction_params(), "completed")
	
	#create new account
	#account = algod.generate_key() #creates random account, comment this code line out
	create_new_account()
	
	#create and sign transaction
	var tx = algod.construct_payment(params, funder_address, account[0], 123456789)
	
	#sending the signed transaction
	var stx = algod.sign_transaction(tx, funder_mnemonic)
	
	#generating the transaction ID
	var txid = yield(algod.send_transaction(stx), "completed")
	
	#wait for confirmation
	print("waiting for confirmation")
	yield(algod.wait_for_transaction(txid), "completed")
	
	# getting the account infromation
	var info = yield(algod.account_information(account[0]), "completed")
	
	#verifying the account's algo holdings
	return info.amount == 123456789

func _test_asset_transfers(): # uses generated account
	print(" -- _test_asset_transfers")

	print("create")
	
	#creates assets
	var tx = algod.construct_asset_create(
		params,
		account[0], # Creator
		"TestCoin",	# Asset name
		2,			# Decimals
		false,		# Default frozen?
		100000,		# Total supply
		"TC"		# Unit name
	)
	var stx = algod.sign_transaction(tx, account[1])
	
	#sending signed transaction
	var txid = yield(algod.send_transaction(stx), "completed")
	var tx_info = yield(algod.transaction_information(txid), "completed") #returns null parameters and braks code
	
	
	print(account[0], '////',account[1]) # account 0 is account creator, accout 1 is mnemonic
	
	var wait= yield(algod.wait_for_transaction(txid), "completed")
	
	if tx_info.get("asset-index") != null:# Error catcher 1 // Asset Index returns null by default
		var asset_index = int(tx_info.get("asset-index")) #non existent int constructor?

		print("opt in")
		var optin_tx = algod.construct_asset_opt_in(
			params,
			funder_address,
			asset_index
		)
		stx = algod.sign_transaction(optin_tx, funder_mnemonic)
		yield(algod.send_transaction(stx), "completed")

		print("atomic swap")

		var algo_tx = algod.construct_payment(
			params,
			funder_address,
			account[0],
			100
		)

		var asset_tx = algod.construct_asset_xfer(
			params,
			account[0],
			funder_address,
			1,
			asset_index
		)

		var txns = algod.group_transactions([algo_tx, asset_tx])
		txns[0] = algod.sign_transaction(txns[0], funder_mnemonic)
		txns[1] = algod.sign_transaction(txns[1], account[1])

		yield(algod.send_transactions(txns), "completed")

		var info = yield(algod.account_information(funder_address), "completed")

		var funder_assets = info.assets
		for asset in funder_assets:
			if asset["asset-id"] == asset_index && asset["amount"] == 1:
				return true

		printerr("   !! _test_asset_transfers failed")
		return false
	else:
		_timeout(7)


func _on_Timer_timeout():
	_timeout(7)

func create_new_account():
	#create new account
	account = algod.generate_key() 
	print("New Account Details: ",account[0], '////',account[1]) # account 0 is account creator, accout 1 is mnemonic
	return account


"# sends transaction btw two accounts"
func _test_transaction_to_receiver_addr(): #works
	print(" -- _sending_transaction")
	
	print("sending tx")
	#get suggested parameters
	params = yield(algod.suggested_transaction_params(), "completed")
	
	#create new account
	#account = algod.generate_key() #creates random account, comment this code line out
	
	#create and sign transaction
	tx = algod.construct_payment(params, funder_address, receivers_address, 123456789)
	
	#sending the signed transaction
	stx = algod.sign_transaction(tx, funder_mnemonic)
	
	#generating the transaction ID
	var txid = yield(algod.send_transaction(stx), "completed")
	
	#wait for confirmation
	print("waiting for confirmation")
	yield(algod.wait_for_transaction(txid), "completed")
	
	# getting the account infromation
	var info = yield(algod.account_information(receivers_address), "completed")
	
	#verifying the account's algo holdings
	return info.amount == 123456789

func _test_asset_transfers_to_receivers_address(): # 
	print(" -- _test_asset_transfers")
	
	
	
	
	#you can set parameters fee, but i opted out 
	
	params = yield(algod.suggested_transaction_params(), "completed")
	
	#creates assets
	create_assets(funder_address) 
	
	#____________________________
	# Whichever account creates the asset must sign the raw transaction
	#____________________________
	
	
	#debug Tx details
	#print (tx)
	
	#generates Raw signed transaction
	#stx = algod.sign_transaction(tx, receivers_mnemonic)
	stx = algod.sign_transaction(tx, funder_mnemonic)
	
	#print (stx)#for debug purposes only #works
	#_check_account_information(funder_address, funder_mnemonic) #debugger
	
	
	
	#Generating transaction Id from signed transaction
	txid = yield(algod.send_transaction(stx), "completed") #breaks and returns null if account doesnt have asset
	
	print (txid)
	#wait for transaction to finish sending
	
	var wait= yield(algod.wait_for_transaction(txid), "completed") #returns null if account doesn't have asset
	
	var tx_info = yield(algod.transaction_information(txid), "completed") #returns null parameters (fixed)
	
	#print (tx_info) #for debug purposes only #returns null value
	
	#_check_account_information(funder_address, funder_mnemonic,"")
	#print(account[0], '////',account[1]) # account 0 is account creator, accout 1 is mnemonic
	#if tx_info.get("asset-index") != null:# Error catcher 1 // Asset Index returns null by default
	var asset_index = int(tx_info.get("asset-index")) #non existent int constructor?

	print("opt in")
	var optin_tx = algod.construct_asset_opt_in(
		params,
		funder_address,
		asset_index
		)
	stx = algod.sign_transaction(optin_tx, funder_mnemonic)
	yield(algod.send_transaction(stx), "completed")



	print("atomic swap")

# constructs new transactions
	var algo_tx = algod.construct_payment(
		params,
		funder_address,
		receivers_address,
		100
	)

	var asset_tx = algod.construct_asset_xfer(
		params,
		receivers_address,
		funder_address,
		1,
		asset_index
	)

	# Sends grouped transactions
	var txns = algod.group_transactions([algo_tx, asset_tx])
	
	#//bug 1: at least one signature did not pass verification. debug the asset creator
	#//bug 2: The asset creator in the create transactions func is different when checked in sandbox. Debug asset creator
	# Both accounts sign transaction 	#bug: at least one signature did not pass verification
	txns[0] = algod.sign_transaction(txns[0], funder_mnemonic)
	txns[1] = algod.sign_transaction(txns[1], receivers_mnemonic)
#----------------------------------------------
	# send signed transaction
	yield(algod.send_transactions(txns), "completed") 
	
	# generate a new transaction ID from the grouped transaction
	#print (txns[0])
	#sddfgs
	# wait for transaction to process
	#var wait_2= yield(algod.wait_for_transaction(txid), "completed")

#-------------------------------------------------------------
	# gets account information as a dictionary
	var info = yield(algod.account_information(funder_address), "completed") #should contain account mnemonic?


	#_check_account_information(funder_address, funder_mnemonic,"assets") #doesnt work here #for debug purposes only
	#_check_account_assets(receivers_address) #for debug purposes only

	var funder_assets = info.assets
	for asset in funder_assets:
		if asset["asset-id"] == asset_index && asset["amount"] == 100000: #condition wil always comes out false #commenting out
		#if asset["asset-id"] && asset_index && asset["amount"] != null:
			return true #asset ID cannot be equal to asset_index and amount cannot be one?

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

#func _check_account_assets(address, mnemonic): # account asset debugger #duplicate of code above?
#	var p = yield(algod.account_information(address, mnemonic), "completed")


func create_assets( address): #works # breaks when not using default sandbox creator acct
	print("-----creating asset----")
	tx = algod.construct_asset_create( #breaks
		params,
		address, # Creator #SDK uses default sandbox wallet and ignores this creator (fixed)
		"GameTestCoin",	# Asset name
		2,			# Decimals #i.e how many decimals from the total supply
		false,		# Default frozen?
		100000,		# Total supply # This is 1000.00
		"GTC"		# Unit name
	)
	return tx

"Placeholder Functions"
func encrypt():
	pass

func decrypt():
	pass
