# Build, copy dll, and run gdscript integration tests
Push-Location $PSScriptRoot 
Remove-Item "./test/project/addons/algodot/lib/*.*" -ErrorAction SilentlyContinue
Remove-Item "./target/build/*" -Recurse -ErrorAction SilentlyContinue
Copy-Item -Path "./test/project/addons" -Destination "./target/build" -Recurse

# Build for windows
cargo build -Z unstable-options --release --out-dir "./test/project/addons/algodot/lib"

# Build for linux with cross
cross build --target x86_64-unknown-linux-gnu -Z unstable-options --release --out-dir "./test/project/addons/algodot/lib" -v

# Uncomment to run gdscript integration tests as in `./test/test.ps1`
#& $Env:GODOT_EXE_PATH --path ./test/project/

Pop-Location