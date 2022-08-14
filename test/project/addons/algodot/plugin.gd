tool
extends EditorPlugin

var gdn


var _editor_view

func _init():
	add_autoload_singleton('DocsHelper', "res://addons/algodot/Documentation/Scripts/DocsHelper.gd")
	#*********For Built in Documentation**********#
	
	_add_custom_editor_view()
	
	"Adds a Custom Tab for Documentations"

	
	get_editor_interface().get_inspector().get_parent_control().get_parent().add_child(_editor_view)
	
	


func _enter_tree():
	

	
	var gui = get_editor_interface().get_base_control()
	var node_icon = gui.get_icon("Node", "EditorIcons")

	add_custom_type(
		"Algod",
		"Node",
		preload("res://addons/algodot/gdnative/algod.gdns"),
		node_icon
	)
	
	
	add_autoload_singleton("AsyncExecutorDriver", "res://addons/algodot/gdnative/async_executor.gdns")
	

	
	

	




func _exit_tree():
	
	remove_custom_type("Algod")
	remove_autoload_singleton("AsyncExecutorDriver")

#************For Builtin Documentation***********#
	
	remove_autoload_singleton("DocsHelper")
	_remove_custom_editor_view()




#***********For Builtin Documentation***********#
func get_plugin_name():
	return "Algodot"


func _add_custom_editor_view(): 
	_editor_view = preload("res://addons/algodot/Documentation/Scripts/DocumentationViewer.tscn").instance()

	


func _remove_custom_editor_view():
	if _editor_view:
		
		remove_control_from_docks(_editor_view)

