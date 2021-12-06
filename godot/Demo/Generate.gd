extends Control

var account

func _on_Generate_pressed():
	# Also disable test if wallet already generated
	$Generate.disabled = true
	$Next.disabled = false
	var d = Directory.new()
	
	d.open("user://")
	d.list_dir_begin(true)
	var address_file = d.get_next()
	while address_file != "":
		if address_file.ends_with(".mnemonic"):
			var f = File.new()
			f.open("user://%s" % address_file, File.READ)
			var mnemonic = f.get_as_text()
			f.close()
			var address = address_file.trim_suffix(".mnemonic")
			account = [address, mnemonic]
		address_file = d.get_next()
	d.list_dir_end()
	
	if !account:
		account = get_node("/root/DemoGame/Algod").generate_key()
		var f = File.new()
		f.open("user://%s.mnemonic" % account[0], File.WRITE)
		f.store_string(account[1])
		f.close()
	


	$Result/RichTextLabel.text = """-- ADDRESS: --
%s
-- MNEMONIC: --
%s

You don't need to copy this for this demo.
""" % [account[0], account[1]]

	get_node("/root/DemoGame/").address = account[0]
	get_node("/root/DemoGame/").mnemonic = account[1]

	
