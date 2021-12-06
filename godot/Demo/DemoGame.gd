extends Node


var address: String setget set_address
var mnemonic: String
var params

func _on_PlayArea_input_event(viewport, event, shape_idx):
	if event is InputEventMouseButton and event.pressed:
		print("pressed")
		$Player.move_to(event.position)


func test_connection():
	$Algod.health()


func _on_Algod_health(response):
	if response == OK:
		$Setup/Panel/Connect.set_result(true, "Connection success!")
		$Algod.suggested_transaction_params()
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
	
	# Create smart contracts
	
	# Load .TEAL file
	# Send to algod to create

func _on_Algod_suggested_transaction_params(response):
	params = response
