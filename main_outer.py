import traceback    #Exceptionnachricht vollständig wenn Programm als .exe abschmiert
import KFS.log
from main import main


try:
    main()
except:
    KFS.log.write(traceback.format_exc())
    
    print("\n\nPress enter to close program.")
    input() #pause