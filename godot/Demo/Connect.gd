extends Control

func _on_Test_pressed():
	get_node("/root/DemoGame").test_connection()
	$Result/RichTextLabel.text = "Testing connection..."

func set_result(res: bool, text: String):
	if res:
		$Next.disabled = false
	$Result/RichTextLabel.text = text
