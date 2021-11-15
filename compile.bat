del ".\nHentai to PDF.exe"

pyinstaller --onefile "main.py" --clean

timeout -t 5
move ".\dist\main.exe" ".\nHentai to PDF.exe"
rmdir /S /Q ".\__pycache__\"
rmdir /S /Q ".\build\"
rmdir /S /Q ".\dist\"
del ".\main.spec"
del ".\debug.log"

pause