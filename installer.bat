cargo build --release -p bin_app --bin bin_app

copy target\release\bin_app.exe %USERPROFILE%\AppData\Local\Microsoft\WindowsApps\mkt.exe

mkt add -p "default-templates/txml.xml" -r