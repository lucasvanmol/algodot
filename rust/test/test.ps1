Push-Location $PSScriptRoot 

cargo build

Copy-Item "../target/debug/algodot.dll" -Destination "./project/lib"

# For now this is set manually with `./sandbox goal account export -a ...`, based on genesis.json
$Env:ALGODOT_FUNDER_MNEMONIC = "letter nasty produce hidden confirm sad color diamond allow ring truth code mirror atom obscure this opinion one life travel chat lobster cook about flight"

& 'C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe' --path ./project/

Pop-Location