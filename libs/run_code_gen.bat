@if "%~1"=="" @echo BHD v1.0, build script for rust bindgen
@echo Building RUST code
@bindgen libbhd.h -o ffi.rs
@if ERRORLEVEL 0 @echo Done, output:ffi.rs