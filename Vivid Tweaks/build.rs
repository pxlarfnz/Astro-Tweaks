name: Build Vivid Tweaks

on:
  push:
    branches: [ "main" ]
  workflow_dispatch:   # allows manual run

jobs:
  build:
    runs-on: windows-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: x86_64-pc-windows-msvc

    - name: Build Release
      run: cargo build --release

    - name: Upload the .exe
      uses: actions/upload-artifact@v4
      with:
        name: Vivid-Tweaks
        path: target/release/vivid-tweaks.exe