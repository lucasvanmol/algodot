extends Node


var address: String setget set_address
var mnemonic: String
var params

func _on_PlayArea_input_event(viewport, event, shape_idx):
	if event is InputEventMouseButton and event.pressed:
		print("pressed")
		$Player.move_to(event.position)


func test_connection():
	var response = yield($Algod.health(), "completed")

	if response == OK:
		$Setup/Panel/Connect.set_result(true, "Connection success!")
		params = yield($Algod.suggested_transaction_params(), "completed")
	else:
		$Setup/Panel/Connect.set_result(false, "Connection failed!")

func set_address(addr):
	address = addr
	$HUD/Address.text = "Your address: " + addr

func set_up_shop():
	# Make ASA
	var tx = $Algod.construct_asset_create(
		params, 
		address,
		"MyCoin", 
		2,
		false,
		1000000,
		"MC",
		null, null, null, null, null, null)
		
	var stxn = $Algod.sign_transaction(tx, mnemonic);
		
	$Algod.send_transaction(stxn)
	
	var payment_asa_id = 0
	var sword_cost = 0
	var sword_asa_id = 0
	# Create smart contracts
	var f = File.new()
	f.open("res://Demo/TEAL/shop.TEAL")
	var teal = f.get_as_text()
	f.close()
	
	var response = yield($Algod.compile_teal(teal), "completed")
	var escrow_address = response[0]
	var bytes = response[1]
	
	# Load .TEAL file
