@if "%~1"=="" @echo BHD v1.0, build script v1.0
@echo Building Win64 executable
@cargo build --release
@if ERRORLEVEL 0 @echo Done, output:../build/bhd_backend.dll