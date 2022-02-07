import datetime as dt
import math


line_last_len=0 #Zeile letzte Länge um evt. zu überschreiben
def write(text: str, prepend_newline: bool=True, UNIX_time: bool=False) -> None:
    global line_last_len


    try:
        text=str(text)
    except ValueError:
        raise TypeError("Error in KFS::log::write(...): Type of \"text\" must be str or convertable to str.")
    if type(prepend_newline)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"prepend_newline\" must be bool.")
    if type(UNIX_time)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"UNIX_time\" must be bool.")


    DT_now=dt.datetime.now(dt.timezone.utc)
    if UNIX_time==False:
        text=f"[{DT_now.strftime('%Y-%m-%dT%H:%M:%SZ')}] {text}"                #Zeitstempel hinzufügen
    else:
        text=f"[{math.floor(DT_now.timestamp()):,.0f}] ".replace(",", ".")+text #Zeitstempel im Unixformat hinzufügen
    

    if len(text)<=23 or text[23]!="\r":         #wenn Zeile letzte nicht überschreiben:
        if prepend_newline==True:
            print("\n", end="", flush=True)
        print(f"{text}", end="", flush=True)    #Zeile letzte nicht überschreiben
    else:                                       #wenn Zeile letzte überschreiben:
        text=text[:23]+text[24:]                #Carriage Return entfernen
        print("\r", end="", flush=True)
        for i in range(math.ceil(line_last_len/100)):
            print("                                                                                                    ", end="", flush=True)
        print(f"\r{text}", end="", flush=True)  #Zeile letzte überschreiben
    
    with open(f"{DT_now.strftime('%Y-%m-%d Log.txt')}", "at") as log_file:
        log_file.write(f"{text}\n") #in Datei schreiben
    
    line_last_len=len(text) #Zeilenlänge merken um nächstes Mal evt. zu überschreiben
    return