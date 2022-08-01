tool
extends EditorPlugin

var gdn

#########for built in documentation
var _editor_view
var show_documentation : bool
#var _parts_inspector
#var _export_plugin
func _init():
	add_autoload_singleton('DocsHelper', "res://addons/algodot/Documentation/Scripts/DocsHelper.gd")


func _enter_tree():
	# Algod custom type

	# Get the base icon from the current editor interface/theme
	var gui = get_editor_interface().get_base_control()
	var node_icon = gui.get_icon("Node", "EditorIcons")

	add_custom_type(
		"Algod",
		"Node",
		preload("res://addons/algodot/gdnative/algod.gdns"),
		node_icon
	)
	
	# Driver for executing async rust code in _process
	add_autoload_singleton("AsyncExecutorDriver", "res://addons/algodot/gdnative/async_executor.gdns")
	

	
	

	


func _ready():
	#####for buiilt in documentation
	#if get_editor_interface().is_plugin_enabled("algodot"):
	var _t = Node.new()#placeholder
	
	_add_custom_editor_view()
	
	"Adds a Custom Tab for Documentations"
	get_editor_interface().get_editor_viewport().add_child(_editor_view)
	#_t.add_child(_editor_view)
	#get_editor_interface().get_editor_viewport().add_child(_t)
	
	
	#get_editor_interface().get_inspector().add_child(_editor_view)
	
	#_editor_view.editor_interface = get_editor_interface()
	"set scale"
	#var _1 =_editor_view.get_parent_control()  #parent is a vbox Container
	#_1.set_alignment(1)  # vbox parent is a panel container
	
	#var scale =get_editor_interface().get_editor_viewport().siz
	#print (scale)


#	if Engine.editor_hint:
#		# Force Godot to show the dialogic folder #change documentation
#		get_editor_interface().get_resource_filesystem().scan()


func _exit_tree():
	# Clean up
	remove_custom_type("Algod")
	remove_autoload_singleton("AsyncExecutorDriver")
##################For builtin Documentation
	
	remove_autoload_singleton("DocsHelper")
	_remove_custom_editor_view()
#	remove_inspector_plugin(_parts_inspector)
#	remove_export_plugin(_export_plugin)



###########For Builtin Documentation###################
func get_plugin_name():
	return "Algodot"

#func make_visible(visible):
#	if _editor_view:
#		_editor_view.visible = visible

func _add_custom_editor_view(): # for builtin documentatiion
	_editor_view = preload("res://addons/algodot/Documentation/Scripts/DocumentationViewer.tscn").instance()
	#_editor_view.plugin_reference = self
	


func _remove_custom_editor_view():
	if _editor_view:
		#remove_control_from_bottom_panel(_editor_view)
		remove_control_from_docks(_editor_view)
#		_editor_view.queue_free()
