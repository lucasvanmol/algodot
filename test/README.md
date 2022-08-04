# Integration Testing

## Godot Installation

In order to run integration tests, godot must be installed and the .exe path set in the env var `GODOT_EXE_PATH` (see `test.ps1`).

## Algorand Sandbox

In addition, an Algorand sandox node must be up and running, ideally with the config found in `./sandbox`. See https://github.com/algorand/sandbox for installation and configuration.

> There is currently a bug in the `algonaut` rust library that causes transactions with their first valid round ("fv") set to 0 to not have this value show up upon serialization. When using a node in release mode, you'll have to wait atleast one block time (4.5s) after starting up the sandbox for the first time. In dev mode (as is the case when using the provided config file `config.test`), blocks are submitted instantly, and are only submitted when transaction are sent, so you'll have to send atleast one transaction when you start up the network for the first time. The signed transaction binary file  `test.stxn` is ready to be POSTed to the network at the endpoint `/v1/transactions`, provided the given genesis.json file is used, or sent with `goal clerk rawsend -f ./test.stxn` 