# PowerShell Integration Testing
#   - Godot must be installed and the exe path set in the env var below
#   - Algorand Sandbox must be up and running, ideally with the config found in `./sandbox`
#   - There is currently a bug in the `algonaut` rust library that causes transactions with their first 
#       valid round ("fv") set to 0 to not have this value show up upon serialization. When using release mode, 
#       you'll have to wait atleast one block time (4.5s) after starting up the sandbox for the first time. 
#       In dev mode (as is the case when using the provided config file `config.test`), blocks are submitted
#       instantly, and are only submitted when transaction are sent, so you'll have to send atleast one 
#       transaction when you start up the network for the first time. The signed transaction binary file 
#       `test.stxn` is ready to be POSTed to the network at `/v1/transactions`, provided the given genesis.json 
#       file is used, or sent with `goal clerk rawsend -f ./test.stxn`
#   - If anyone ends up making a bash script or similair for linux, feel free to PR 

# Location of godot install
$Env:GODOT_EXE_PATH = "C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe"

# Account used to fund tests,
# For now this is set manually with `./sandbox goal account export -a ...`
# This particular mnemonic corresponds to the address `S5THZ2PD5POQGXAEAGUBPUM3X5NLJWBBBCVXK7WLSH6DMEEW6UJB45TBNE` found in genesis.json
$Env:ALGODOT_FUNDER_MNEMONIC = "letter nasty produce hidden confirm sad color diamond allow ring truth code mirror atom obscure this opinion one life travel chat lobster cook about flight"

# Build, copy dll, and run gdscript integration tests
Push-Location $PSScriptRoot 
cargo build
Copy-Item "../target/debug/algodot.dll" -Destination "./project/lib"
& $Env:GODOT_EXE_PATH --path ./project/
Pop-Location