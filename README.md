# Algodot
Algorand integration in Godot.


## Download the addon

https://godotengine.org/asset-library/asset/1219

https://inhumanity-arts.itch.io/algodot

## Usage

This addon comes with prebuilt Logic scripts in Algod.gd that can be used in your godot project and 
Built-in documentation on how to use them in your scene's viewport tab. But should you decide to construct 
additional logic for your project, here are some valid examples

Initializing the Algod object for Local Testnet

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

## Building on Windows

```
./build.ps1
```

On windows and Linux, there maybe some errors with bindgen not finding C headers: see https://godot-rust.github.io/book/faq/configuration.html#c-headers-not-found-by-bindgen. It's advisable to built in a fresh docker environment for optimal builds

Running in the Visual Studio Developer Console should set the appropriate env vars, notable one called `INCLUDE`. For my installation, this looked something like: 

```
INCLUDE=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.8\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.19041.0\cppwinrt
```

#Android

This fork contains a tested Android Build CI using cargo ndk and  openssl 1.1.1 android. It uploads them as artifacts, or optionally you can download prebuilt binaries for Windows, Linux and Android armv7 architecture through the links listed above. The android build currently only supports armv7 and arm64-v8a. If compiling from source, be sure to rename openssl dependencies libssl.so and libcrypto.so into libssl.so.1.1 and libcrypto.so.1.1, and them including them as dependencies in algodot gdnlib.tres file for both Android architectures.

#Dependencies

Linux and Android builds come with prebuilt dependencies for openssl libssl.so and libcrypto.so. These are needed for Algodot to commmunicate with the Algorand Blockchain. If compiling for macOS and Windows architecture, be sure to have openssl preinstalled in your computer or supply them as dependencies in algodot's gdnlib.tres file.



![Screenshot](https://github.com/Sam2much96/algodot/blob/master/test/project/addons/algodot/icon.png)
