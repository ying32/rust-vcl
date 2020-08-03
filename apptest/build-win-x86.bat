@echo off

set destDir=.\target\i686-pc-windows-msvc\debug

copy ".\win32\liblcl.lib" ".\liblcl.lib"

cargo rustc --target=i686-pc-windows-msvc -- -C link-args="/MANIFEST /SAFESEH:NO"

if %errorlevel%==0 (
	if not exist "%destDir%\liblcl.dll" (copy ".\win32\liblcl.dll" "%destDir%\liblcl.dll") 
	if not exist "%destDir%\applogo.ico" (copy ".\applogo.ico" "%destDir%\applogo.ico") 
	if exist "%destDir%\apptest.exe" (start "" "%destDir%\apptest.exe") 
) else (
	pause
)

