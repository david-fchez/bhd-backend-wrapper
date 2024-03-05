@if "%~1"=="" @echo BHD v1.0, build script v1.0
@echo Building Android executable
@cargo build --release --target x86_64-linux-android	
REM @cargo build --release --target armv7-linux-androideabi	The last android phone with armv7 32 bit was released in 2014
@cargo build --release --target aarch64-linux-android	
@cargo build --release --target i686-linux-android	
@if ERRORLEVEL 0 @echo Done, output:../build/bhd_backend.dll