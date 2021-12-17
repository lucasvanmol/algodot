tool
extends EditorPlugin

var gdn

func _enter_tree():
	_setup_async()

	var run_tests = false
	for arg in OS.get_cmdline_args():
		if arg == "--run-editor-tests":
			run_tests = true
			break
	if run_tests:
		_run_tests()
	else:
		print("Opening editor normally for the test project. To run tests, pass `--run-editor-tests` to the executable.")

func _setup_async():
	# Algod custom type

	# Get the base icon from the current editor interface/theme
	var gui = get_editor_interface().get_base_control()
	var node_icon = gui.get_icon("Node", "EditorIcons")

	add_custom_type(
		"Algod",
		"Node",
		preload("res://gdnative/algod.gdns"),
		node_icon
	)
	
	# Driver for executing async rust code in _process
	add_autoload_singleton("AsyncExecutorDriver", "res://gdnative/async_executor.gdns")

func _run_tests():
	print(" -- Rust gdnative test suite:")
	gdn = GDNative.new()
	var status = false;

	gdn.library = load("res://gdnative.gdnlib")

	if gdn.initialize():
		status = gdn.call_native("standard_varcall", "run_tests", [])

		gdn.terminate()
	else:
		print(" -- Could not load the gdnative library.")

	if status:
		print(" -- Test run completed successfully.")
	else:
		print(" -- Test run completed with errors.")
		OS.exit_code = 1

	print(" -- exiting.")
	get_tree().quit()


func _exit_tree():
	# Clean up
	remove_custom_type("Algod")
	remove_autoload_singleton("AsyncExecutorDriver")
