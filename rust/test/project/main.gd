extends Node

var gdn

func _ready():
	print(" -- Rust gdnative test suite:")
	_timeout()

	var status = yield(_test_algod(), "completed")


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


func _test_algod():
	print(" -- _test_algod")

	var algod = Algod.new()
	algod.url = "http://localhost:4001"
	algod.token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
	add_child(algod)
	
	var addr = algod.generate_key()
	
	print(addr)
	var status = true
	
	# Force this to return a FunctionState for convenience
	yield(get_tree().create_timer(0.1), "timeout")
	
	status = status && (yield(algod.health(), "completed") == OK)
	
	if !status:
		printerr("   !! _test_algod failed")

	return status
