import traceback    #Exceptionnachricht vollständig wenn Programm als .exe abschmiert
from KFS import log
from main import main


try:
    main()
except:
    log.write(f"CRASH\n{traceback.format_exc()}")
    
    print("\n\nPress enter to close program.")
    input() #pause