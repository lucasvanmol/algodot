Push-Location $PSScriptRoot 

cargo build
Copy-Item "../target/debug/algodot.dll" -Destination "./project/lib"
& 'C:\Program Files (x86)\Steam\steamapps\common\Godot Engine\godot.windows.opt.tools.64.exe' --path ./project/

Pop-Location