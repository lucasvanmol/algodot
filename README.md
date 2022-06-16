# Algodot
Algorand integration in Godot

## Download the addon

https://godotengine.org/asset-library/asset/1219

## Usage

Initializing the Algod object

```gdscript
algod = Algod.new()
algod.url = "http://localhost:4001"
algod.token = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
add_child(algod)
```

Test connections using `.health()`

```gdscript
assert(yield(algod.health(), "completed") == OK)
```

Sending transactions

```gdscript
var from_mnemonic = "your twenty five word mnemonic ..."
var from_address = algod.get_address(from_mnemonic)

# Get suggested parameters
var params = yield(algod.suggested_transaction_params(), "completed")

# Generate a new account
var to_account = algod.generate_key()

# Create and sign transaction
var tx = algod.construct_payment(params, from_address, account[0], 123456789)
var stx = algod.sign_transaction(tx, from_mnemonic)
var txid = yield(algod.send_transaction(stx), "completed")

# Wait for confirmation
yield(algod.wait_for_transaction(txid), "completed")
var info = yield(algod.account_information(account[0]), "completed")

assert(info.amount == 123456789)
```
	
For more examples, check out the test script in the `./test/project` directory.

# Development

## Testing

See the README in `./test`

## Building

```
./build.ps1
```