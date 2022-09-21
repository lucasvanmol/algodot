# Algodot
Algorand integration in Godot.
![Screenshot](https://github.com/Sam2much96/algodot/blob/master/test/project/addons/algodot/icon.png)

-Contains Builtin Documentation
-Comes with Prebuilt Godot Alog Class

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

On windows, there maybe some errors with bindgen not finding C headers: see https://godot-rust.github.io/book/faq/configuration.html#c-headers-not-found-by-bindgen

Running in the Visual Studio Developer Console should set the appropriate env vars, notable one called `INCLUDE`. For my installation, this looked something like: 

```
INCLUDE=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
```

#Android

This fork contains a tested Android Build using cargo ndk and  openssl 1.1.1 android. 
