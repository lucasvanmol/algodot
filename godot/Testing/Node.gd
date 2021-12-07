extends Control

func _on_Button_pressed():
	#$Label.text = "Data = " + $Algod.get_data()
#	$Algod.health(funcref(self, "test_callback"))

	var a = yield($Algod.health(), "completed")
	print(a)
 
	#$Algod.suggested_transaction_params()
	#$Algod.account_information("ZW3ISEHZUHPO7OZGMKLKIIMKVICOUDRCERI454I3DB2BH52HGLSO67W754")
	#var account = $Algod.generate_key()
	#var account = ["YOEYQJO25HLTHCVM4ZRHI2WXRKWDCRUSYHC75DQXT6U74ELTKVACDTMBNU", "child accident biology child elder demand scale arrow example core state general obscure harvest bone siege all dream accuse service polar bind trim ability rain"]
	#$Algod.send_transaction(account[1], "ZW3ISEHZUHPO7OZGMKLKIIMKVICOUDRCERI454I3DB2BH52HGLSO67W754", 100000)

	#$Algod.status()



func _on_Algod_transaction_sent(response):
	print("transaction sent:")
	print(response)
	print("----------------------------")


func _on_Algod_account_info(response):
	print("account_info:")
	print(response)
	print("----------------------------")



func _on_Algod_status(response):
	print("status:")
	print(response)
	print("----------------------------")


func _on_Algod_transaction_confirmed(transaction_info):
	print("transaction confirmed")
	print(transaction_info)
	print("----------------------------")


func _on_Algod_suggested_transaction_params(response):
	print("suggested txn params recieved, constructin txn")
	var transaction = $Algod.construct_payment_tx(
		response, 
		"YOEYQJO25HLTHCVM4ZRHI2WXRKWDCRUSYHC75DQXT6U74ELTKVACDTMBNU", 
		"ZW3ISEHZUHPO7OZGMKLKIIMKVICOUDRCERI454I3DB2BH52HGLSO67W754", 
		1000)
	var mnemonic = "child accident biology child elder demand scale arrow example core state general obscure harvest bone siege all dream accuse service polar bind trim ability rain"
	print("sending txn")
	$Algod.send_transaction(transaction, mnemonic)
	
	print("----------------------------")
