# Location of godot install
$Env:GODOT_EXE_PATH = "C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe"

# Account used to fund tests,
# For now this is set manually with `./sandbox goal account export -a ...`
# This particular mnemonic corresponds to the address `S5THZ2PD5POQGXAEAGUBPUM3X5NLJWBBBCVXK7WLSH6DMEEW6UJB45TBNE` found in genesis.json
$Env:ALGODOT_FUNDER_MNEMONIC = "letter nasty produce hidden confirm sad color diamond allow ring truth code mirror atom obscure this opinion one life travel chat lobster cook about flight"

# Build, copy dll, and run gdscript integration tests
Push-Location $PSScriptRoot 

# Remove existing files
Remove-Item "./project/addons/algodot/lib/*.*" -ErrorAction SilentlyContinue

# Build the project
# `--out-dir` requires `-Z unstable-options`
cargo build -Z unstable-options --out-dir "./project/addons/algodot/lib" # --release

# Run the test project
& $Env:GODOT_EXE_PATH --path ./project/

Pop-Location