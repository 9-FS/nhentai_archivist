import os
from KFS import log


def get_h_ID_list():
    h_ID_list=[]    #hentai ID list
    user_input=""

    
    file_tried=False
    while True:
        if os.path.isfile("downloadme.txt")==True and file_tried==False:         #if ID list in file: load from file, only try once
            file_tried=True
            with open("downloadme.txt", "rt") as downloadme_file:
                h_ID_list=[line for line in downloadme_file.read().split("\n") if line!=""] #seperate ID with linebreaks, remove empty lines
            if len(h_ID_list)!=0:
                log.write("downloadme.txt loaded.")
            else:
                log.write("downloadme.txt loaded. Nothing found inside.")
        
        else:   #if ID list file not available: ask user for input
            log.write("Enter the holy numbers: ")
            user_input=input()
            log.write(user_input)
            h_ID_list=user_input.split()    #user input seperated at whitespace
        
        if len(h_ID_list)==0:   #if file or user input empty: retry
            continue

        for i in range(len(h_ID_list)): #convert all ID to int
            try:
                h_ID_list[i]=int(h_ID_list[i], base=10)
            except ValueError:  #if input invalid: discard whole input, ask user (again)
                log.write("Error: Could not convert input \""+h_ID_list[i]+"\" to int.")
                break
        else:   #if all ID converted without failure: break out of while, return desired ID
            break

    return h_ID_list