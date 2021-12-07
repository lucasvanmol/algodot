extends Control



func _on_Info_pressed():
	var address = get_node("/root/DemoGame").address
	var info = yield(get_node("/root/DemoGame/Algod").account_information(address), "completed")

	var amount = info.amount
	$Result/RichTextLabel.text = """You have %d microAlgos in your account. You need at least 100000.
Account Info: 
%s""" % [amount, to_json(info)]

	if amount >= 100_000:
		$Next.disabled = false


