import datetime as dt
import math
import os


line_last_len=0                 #Zeile letzte Länge um evt. zu überschreiben
timestamp_old=""                #Zeitstempel zuletzt ausgegeben
timestamp_old_line_previous=""  #Zeitstempel zuletzt ausgegeben auf Zeile davor
def write(text: str, append_to_line_current: bool=False, UNIX_time: bool=False) -> None:
    global line_last_len
    global timestamp_old
    global timestamp_old_line_previous
    newline_replacement="\n"        #womit soll Zeilenumbruch ersetzt werden? (Zeilenumbruch + Einrückung)
    overwrite_line_current=False    #Zeile letzte überschreiben?
    timestamp=""                    #Zeitstempel vor Logeintrag, entweder richtiger Zeitstempel oder Leerzeichen wenn gleich wie davor
    timestamp_in_console=""         #Zeitstempel in Konsolenlog hinzufügen ("none", "just spaces", "full")
    timestamp_in_file=""            #Zeitstemp in Logdatei hinzufügen ("none", "just spaces", "full")
    timestamp_new=""                #Zeitstempel aktuell, nur bei Modus "full" verwenden


    try:
        text=str(text)
    except ValueError:
        raise TypeError("Error in KFS::log::write(...): Type of \"text\" must be str or convertable to str.")
    if type(append_to_line_current)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"append_to_line_current\" must be bool.")
    if type(UNIX_time)!=bool:
        raise TypeError("Error in KFS::log::write(...): Type of \"UNIX_time\" must be bool.")


    DT_now=dt.datetime.now(dt.timezone.utc)                                         #Zeitpunkt aktuell
    if UNIX_time==False:                                                            #wenn nicht im Unix-Format:
        timestamp_new=f"[{DT_now.strftime('%Y-%m-%dT%H:%M:%SZ')}]"                  #Zeitstempel neu nach ISO8601
    else:
        timestamp_new=f"[{math.floor(DT_now.timestamp()):,.0f}]".replace(",", ".")  #Zeitstempel neu im Unixformat
    
    os.makedirs("./Log/", exist_ok=True)    #Log-Ordner erstellen

    if text[0:1]=="\r":             #wenn Zeichen [0] Carriage Return: Zeile letzte überschreiben, Inhalte vorher löschen
        overwrite_line_current=True #Zeile letzte überschreiben
        print("\r", end="", flush=True)
        for i in range(math.ceil(line_last_len/100)):
            print("                                                                                                    ", end="")
        text=text[1:]               #\r entfernen

    for i in range(len(timestamp_new)+1):           #Einrückungsbreite
        newline_replacement+=" " 
    text=text.replace("\n", newline_replacement)    #Text nach Zeilenumbruch einrücken

    line_last_len=len(timestamp_new)+1+len(text)    #Zeilenlänge merken um nächstes Mal evt. zu überschreiben
    
    if overwrite_line_current==False and append_to_line_current==False and timestamp_old!="":   #wenn Zeile neu:
        timestamp_old_line_previous=timestamp_old                                               #Zeitstempel aktualisieren auf Zeile zuvor
    if timestamp_old_line_previous!=timestamp_new:  #wenn Zeitstempel anders als auf Zeile zuvor ausgegeben:
        timestamp_in_console="full"                 #in Konsole Zeitstempel anzeigen
    else:                                           #wenn Zeitstempel gleich wie in Zeile zuvor ausgegeben:
        timestamp_in_console="just spaces"          #kein Zeitstempel, nur Einrückung
    if timestamp_old!=timestamp_new:        #wenn Zeitstempel anders als zuletzt ausgegeben:
        timestamp_in_file="full"            #in Logdatei Zeitstempel schreiben
    else:                                   #wenn Zeitstempel gleich wie zuletzt ausgegeben:
        timestamp_in_file="just spaces"     #kein Zeitstempel, nur Einrückung

    if overwrite_line_current==True:    #wenn Zeile aktuell überschreiben:
        print("\r", end="")             #in Konsole Carriage Return
        with open(f"./Log/{DT_now.strftime('%Y-%m-%d Log.txt')}", "at") as log_file:    
            log_file.write(f"\n")       #in Datei aber Zeilenumbruch
        timestamp_old=timestamp_new     #Zeitstempel zuletzt ausgegeben aktualisieren
    elif append_to_line_current==True:  #wenn in Zeile aktuell dranhängen:
        timestamp_in_console="none"     #Standardzeitstempeleinstellungen ignorieren, immer kein Zeitstempel, auch keine Einrückung
        timestamp_in_file="none"
    else:                           #wenn Zeile neu:
        print("\n", end="")         #in Konsole Zeilenumbruch
        with open(f"./Log/{DT_now.strftime('%Y-%m-%d Log.txt')}", "at") as log_file:
            log_file.write(f"\n")   #in Datei Zeilenumbruch
        timestamp_old=timestamp_new                 #Zeitstempel zuletzt ausgegeben aktualisieren
   

    if timestamp_in_console=="full":            #wenn Zeitstempel gewünscht:
        timestamp=f"{timestamp_new} "
    elif timestamp_in_console=="just spaces":   #wenn Einrückung gewünscht:
        for i in range(len(timestamp_old)+1):   #Einrückungsbreite
            timestamp+=" "                      #kein Zeitstempel ausgeben, nur Einrückung
    elif timestamp_in_console=="none":          #wenn nix gewünscht:
        timestamp=""
    else:
        raise RuntimeError(f"Error in KFS::log::write(...): timestamp_in_console has invalid value which should not occur (\"{timestamp_in_console}\").")
    print(f"{timestamp}{text}", end="", flush=True)
    timestamp=""    #vor Zeitstempelbestimmung für Datei Zeitstempel zurücksetzen
    
    if timestamp_in_file=="full":               #wenn Zeitstempel gewünscht:
        timestamp=f"{timestamp_new} "
    elif timestamp_in_file=="just spaces":      #wenn Einrückung gewünscht:
        for i in range(len(timestamp_old)+1):   #Einrückungsbreite
            timestamp+=" "                      #kein Zeitstempel ausgeben, nur Einrückung
    elif timestamp_in_file=="none":             #wenn nix gewünscht:
        timestamp=""
    else:
        raise RuntimeError(f"Error in KFS::log::write(...): timestamp_in_file has invalid value which should not occur (\"{timestamp_in_file}\").")
    with open(f"./Log/{DT_now.strftime('%Y-%m-%d Log.txt')}", "at") as log_file:
        log_file.write(f"{timestamp}{text}")

    return