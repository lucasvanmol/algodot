tool
extends EditorPlugin


var _editor_view

func _init():
	add_autoload_singleton('DocsHelper', "res://addons/algodot/Documentation/Scripts/DocsHelper.gd")
	#*********For Built in Documentation**********#
	
	_add_custom_editor_view()
	
	"Adds a Custom Tab for Documentations"
	get_editor_interface().get_editor_viewport().add_child(_editor_view)
	make_visible(false)

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
	#remove_autoload_singleton("AsyncExecutorDriver") #causes leaked memory bug

#************For Builtin Documentation***********#
	
	remove_autoload_singleton("DocsHelper")
	_remove_custom_editor_view()




#***********For Builtin Documentation***********#
func get_plugin_name()-> String:
	return "Algodot"


func _add_custom_editor_view(): 
	_editor_view = preload("res://addons/algodot/Documentation/Scripts/DocumentationViewer.tscn").instance()

	


func _remove_custom_editor_view():
	if _editor_view:
		_editor_view.queue_free()

func has_main_screen()-> bool:
	return true

func make_visible(visible: bool) -> void:
	if _editor_view:
		_editor_view.visible=visible

func get_plugin_icon()-> Texture:
	return get_editor_interface().get_base_control().get_icon("Spatial", "EditorIcons")

