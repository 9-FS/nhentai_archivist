del ".\nHentai to PDF.exe"

pyinstaller --onefile "main_outer.py" --clean

timeout -t 5
move ".\dist\main_outer.exe" ".\nHentai to PDF.exe"
rmdir /S /Q ".\__pycache__\"
rmdir /S /Q ".\build\"
rmdir /S /Q ".\dist\"
del ".\main_outer.spec"
del ".\debug.log"

".\nHentai to PDF.exe"