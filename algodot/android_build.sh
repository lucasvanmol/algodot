#!/usr/bin/env bash


#Edit the Linker Paths to your own Android Sdk folder

export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="/home/samuel/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="/home/samuel/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"
export CARGO_TARGET_I686_LINUX_ANDROID_LINKER="/home/samuel/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android29-clang"
export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER="/home/samuel/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android29-clang"
export ANDROID_SDK_ROOT="/home/samuel/Android/Sdk"
export ANDROID_NDK_VERSION=21.4.7075529

#export OPENSSL_DIR="/usr/bin/openssl"
export PKG_CONFIG_PATH="/home/samuel/algodot/algodot/pkgconfig"

#RUST_BACKTRACE=full DEBUG=true cargo test build --release --target x86_64-linux-android

#RUST_BACKTRACE=full cargo fix --target x86_64-linux-android

#cross build --target x86_64-linux-android -Z unstable-options --release --out-dir "./test/project/addons/algodot/" -v

#CARGO_LOG=debug cargo run #runs a cargo debug 

#RUST_BACKTRSCE=full CARGO_LOG=debug cargo build --target armv7-linux-androideabi  --release -v


#cargo build --release --target x86_64-linux-android -v

cross build --target x86_64-linux-android -v