import datetime as dt
import math


line_last_len=0 #Zeile letzte Länge um evt. zu überschreiben
def write(text: str, append_to_line_current: bool=False, UNIX_time: bool=False) -> None:
    global line_last_len
    newline_replacement="\n"        #womit soll Zeilenumbruch ersetzt werden? (Zeilenumbruch + Einrückung)
    overwrite_line_current=False    #Zeile letzte überschreiben?
    timestamp=""                    #Zeitstempel vor Logeintrag


    try:
        text=str(text)
    except ValueError:
        raise TypeError("Error in KFS::log::write(...): Type of \"text\" must be str or convertable to str.")
    if type(append_to_line_current)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"append_to_line_current\" must be bool.")
    if type(UNIX_time)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"UNIX_time\" must be bool.")


    DT_now=dt.datetime.now(dt.timezone.utc) #Zeitpunkt aktuell

    if text[0:1]=="\r":             #wenn Zeichen [0] Carriage Return: Zeile letzte überschreiben, Inhalte vorher löschen
        overwrite_line_current=True #Zeile letzte überschreiben
        print("\r", end="", flush=True)
        for i in range(math.ceil(line_last_len/100)):
            print("                                                                                                    ", end="", flush=True)
        text=text[1:]               #\r entfernen

    if overwrite_line_current==False and append_to_line_current==False: #wenn Zeile aktuell nicht überschreiben und nicht an Zeile aktuell angehangen werden soll:
        print("\n", end="", flush=True)                     #Zeilenumbruch
    
    if append_to_line_current==False:                                               #wenn nicht einfach an Zeile angehangen werden soll:
        if UNIX_time==False:                                                        #wenn nicht im Unix-Format:
            timestamp=f"[{DT_now.strftime('%Y-%m-%dT%H:%M:%SZ')}]"                  #Zeitstempel nach ISO8601
        else:
            timestamp=f"[{math.floor(DT_now.timestamp()):,.0f}]".replace(",", ".")  #Zeitstempel im Unixformat
        text=f"{timestamp} {text}"                                                  #Zeitstempel hinzufügen
    
    for i in range(len(timestamp)+1):               #Einrückungsbreite
        newline_replacement+=" " 
    text=text.replace("\n", newline_replacement)    #Text einrücken
    
    line_last_len=len(text) #Zeilenlänge merken um nächstes Mal evt. zu überschreiben
    

    if overwrite_line_current==True:
        print("\r", end="")
    print(f"{text}", end="", flush=True)    #Text drucken
    with open(f"{DT_now.strftime('%Y-%m-%d Log.txt')}", "at") as log_file:
        log_file.write(f"{text}\n")         #in Datei schreiben
    return