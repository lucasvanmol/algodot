# *************************************************
# godot3-Dystopia-game by INhumanity_arts
# Released under MIT License
# *************************************************
# Documentation Viewer
# Builtin Documentation Within the Scene Tree
# To Do:
#(1) Connect signals programmatically
# *************************************************

#Main scene for showing the Project's Documentation

tool
extends Control

onready var DocTree = $HSplit/VBoxContainer/DocumentationTree
onready var DocPageViewer = $HSplit/DocsPageViewer


func _on_DocsPageViewer_open_non_html_link(link, section):
	#print ("VVVVVVVVVVVVVVVVVV") #for debug purposes only
	DocTree.select_item(link)
	DocPageViewer.scroll_to_section(section)
	

func _on_DocumentationTree_page_selected(path): # Duplicate of line 29
	#print ("HHHHHHHHHHHHHHHHHHHHHHHHH") #for debug purposes only
	DocPageViewer.load_page(path)

func _on_FilterEntry_text_changed(new_text):
	var child = DocTree.documentation_tree.get_children()
	while child:
		child.call_recursive("call_deferred", "free")
		child = child.get_next()
	DocsHelper.build_documentation_tree(DocTree, DocTree.documentation_tree,{},{}, new_text)
	DocTree.call_deferred("update")


func _on_DocumentationTree__page_selected(path):
	#print ("UUUUUUUUUUUUUUUUUUUUUUUU") #for debug purposes only
	_on_DocumentationTree_page_selected(path)
