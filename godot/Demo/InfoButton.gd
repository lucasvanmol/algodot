extends Button


export(String, MULTILINE) var info_text


# Called when the node enters the scene tree for the first time.
func _ready():
	$WindowDialog/Label.text = info_text


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_TextureButton_pressed():
	$WindowDialog.popup()
