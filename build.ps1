# Build, copy dll, and run gdscript integration tests
Push-Location $PSScriptRoot 
Remove-Item "./test/project/addons/algodot/lib/*.*" -ErrorAction SilentlyContinue
Remove-Item "./target/build/*" -Recurse -ErrorAction SilentlyContinue
Copy-Item -Path "./test/project/addons" -Destination "./target/build" -Recurse
docker build -t algodot-buildtarget/gnu:0.2.1 ./docker
cargo build -Z unstable-options --release --out-dir "./test/project/addons/algodot/lib"
cross build --target x86_64-unknown-linux-gnu  -Z unstable-options --release --out-dir "./test/project/addons/algodot/lib"
#& $Env:GODOT_EXE_PATH --path ./test/project/
Pop-Location