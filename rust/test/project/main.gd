extends Node

var algod: Algod
var funder_mnemonic
var funder_address
var account
var params

func _ready():
	print(" -- Rust gdnative test suite:")
	_timeout()

	var status = true

	print(" -- Initialize Algod")
	algod = Algod.new()
	algod.url = "http://localhost:4001"
	algod.token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
	add_child(algod)
	
	print(" -- Get funder account")
	funder_mnemonic = OS.get_environment("ALGODOT_FUNDER_MNEMONIC")
	funder_address = algod.get_address(funder_mnemonic)
	if funder_mnemonic == "":
		push_error("Env var `ALGODOT_FUNDER_MNEMONIC` not set")
		status = false

	status = status && yield(_test_algod_connection(), "completed")
	status = status && yield(_test_transaction(), "completed")
	status = status && yield(_test_asset_transfers(), "completed")


	if status:
		print(" -- Test run completed successfully.")
	else:
		print(" -- Test run completed with errors.")
		OS.exit_code = 1

	print(" -- exiting.")
	get_tree().quit()

func _timeout():
	yield(get_tree().create_timer(10.0), "timeout")
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
	
func _test_transaction():
	print(" -- _test_transaction")
	
	print("sending tx")
	params = yield(algod.suggested_transaction_params(), "completed")
	account = algod.generate_key()
	var tx = algod.construct_payment(params, funder_address, account[0], 123456789)
	var stx = algod.sign_transaction(tx, funder_mnemonic)
	var txid = yield(algod.send_transaction(stx), "completed")
	
	print("waiting for confirmation")
	yield(algod.wait_for_transaction(txid), "completed")
	var info = yield(algod.account_information(account[0]), "completed")
	
	return info.amount == 123456789

func _test_asset_transfers():
	print(" -- _test_asset_transfers")

	print("create")
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
	var txid = yield(algod.send_transaction(stx), "completed")
	var tx_info = yield(algod.transaction_information(txid), "completed")
	var asset_index = int(tx_info.get("asset-index"))

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


	return true
