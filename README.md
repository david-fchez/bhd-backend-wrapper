# BHD backend wrapper
This project is used as a bridge between the Godot BHD game and the backend written in golang. The project is written in Rust.

## Dependencies
To build this projects you need:
* `rust-bindgen` (https://rust-lang.github.io/rust-bindgen/)
* `godot-rust` (https://godot-rust.github.io/)

## Building
When a new go method is exprted in the backend project, you need to get the .h header file from the go build folder and copy it to `/backend_wrapper/libs`
Then run `bindgen libbhd.h -o ffi.rs` or `run_code_ben.bat` to generate the rust ffi interface to use go

To build the backend_wrapper libs you need to run 
```bash
cargo build --release
```
the output is in /target/release

If you are building for another platform pass a param to the build command like: 
```bash
cargo build --release --target aarch64-linux-android"
```

Copy the genereted libs files for each architecture in the corresponding libs folder in /frontend/libs
