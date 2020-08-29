@echo off

set libPath=.\win32
set PATH=%libPath%;%PATH%
set LIB=%LIB%;%libPath%

cargo run --target=i686-pc-windows-msvc

rem cargo rustc --target=i686-pc-windows-msvc -- -C link-args="/MANIFEST /manifest:embed /manifestinput:app.manifest /SAFESEH:NO /MACHINE:X86 app.res"
