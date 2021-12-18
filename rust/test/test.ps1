Push-Location $PSScriptRoot 

cargo build

Copy-Item "../target/debug/algodot.dll" -Destination "./project/lib"

# For now this is set manually with `./sandbox goal account export -a ...`
$Env:ALGODOT_FUNDER_MNEMONIC = "version hand outdoor sting spawn warrior noise rose lift faint apart shaft clerk melt deal soft shop pool sister collect armed planet mixed absent unfair"

& 'C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe' --path ./project/

Pop-Location