extends Control

#var driver
#
#func _ready():
#	driver = AsyncExecutorDriver.new()

func _on_Button_pressed():
	$Label.text = "Data = " + $Algod.get_data()
	$Algod.suggested_transaction_params()

#func _process(delta):
#	if not Engine.editor_hint:
#		driver._process(delta)
#
#func _exit_tree():
#	driver.queue_free()
