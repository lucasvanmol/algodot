tool
extends EditorPlugin


func _enter_tree():
	# Algod custom type
	var node_icon = preload("res://addons/algodot/algorand_logo.png")

	add_custom_type(
		"Algod",
		"Node",
		preload("res://addons/algodot/gdnative/algod.gdns"),
		node_icon
	)
	
	# Driver for executing async rust code in _process
	add_autoload_singleton("AsyncExecutorDriver", "res://addons/algodot/gdnative/async_executor.gdns")


func _exit_tree():
	# Clean up
	remove_custom_type("Algod")
	remove_autoload_singleton("AsyncExecutorDriver")


