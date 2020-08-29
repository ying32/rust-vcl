@echo off

set libPath=.\win64
set PATH=%libPath%;%PATH%
set LIB=%LIB%;%libPath%

cargo run --target=x86_64-pc-windows-msvc
 
rem cargo rustc --target=x86_64-pc-windows-msvc -- -C link-args="/MANIFEST /manifest:embed /manifestinput:app.manifest /SAFESEH:NO /DYNAMICBASE:NO /MACHINE:X64 app.res"
 