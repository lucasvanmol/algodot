extends Control

func _ready():
	for child in $Panel.get_children():
		child.hide()
	
	$Panel/Connect.show()

var step = 0

func _on_Next_pressed():
	step += 1
	if step == 1:
		$Panel/Connect.hide()
		$Panel/Generate.show()
	elif step == 2:
		$Panel/Generate.hide()
		$Panel/Fund.show()
	elif step == 3:
		$Panel/Fund.hide()
		$Panel/SetUpShop.show()
		
