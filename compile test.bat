del ".\test.exe"

pyinstaller --onefile "test.py" --clean

timeout -t 5
move ".\dist\test.exe" ".\test.exe"
rmdir /S /Q ".\__pycache__\"
rmdir /S /Q ".\build\"
rmdir /S /Q ".\dist\"
del ".\test.spec"
del ".\debug.log"