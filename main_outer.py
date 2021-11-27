import datetime as dt
import math
import traceback    #Exceptionnachricht vollständig wenn Programm als .exe abschmiert
from main import main


try:
    main()
except:
    DT_now=dt.datetime.now(dt.timezone.utc)
    Crash_Report="--------------------------------------------------\n"
    Crash_Report+=f"{DT_now.strftime('%Y-%m-%dT%H:%M:%S')} | {math.floor(DT_now.timestamp()):,.0f}\n".replace(",", ".")
    Crash_Report+=traceback.format_exc()

    with open(DT_now.strftime("%Y-%m-%d Crash Reports.txt"), "at", encoding="utf-8") as Crash_File:
        Crash_File.write(Crash_Report)
    print("\n\n"+Crash_Report)
    
    print("Press enter to close program.")
    input() #pause