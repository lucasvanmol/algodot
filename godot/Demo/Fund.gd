extends Control

func _ready():
	get_node("/root/DemoGame/Algod").connect("account_info", self, "_on_account_info")

func _on_Info_pressed():
	var address = get_node("/root/DemoGame").address
	get_node("/root/DemoGame/Algod").account_information(address)
	
func _on_account_info(info):
	var amount = info.amount
	$Result/RichTextLabel.text = """You have %d microAlgos in your account. You need at least 100000.
Account Info: 
%s""" % [amount, to_json(info)]

	if amount >= 100_000:
		$Next.disabled = false


