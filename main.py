import datetime as dt
import math
import os
import shutil   #remove tree
import time
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai    import download_hentai
from get_h_ID_list      import get_h_ID_list
from KFS                import log


def main():
    h_ID_list=[]    #hentai ID to download


    log.write("--------------------------------------------------")
    
    h_ID_list=get_h_ID_list()                   #get desired hentai ID
    
    i=0
    i_changed=True  #i changed since last iteration, for console printouts -------
    while i<len(h_ID_list): #work through all desired hentai
        if i_changed==True:
            log.write("--------------------------------------------------")
            log.write(f"{i+1:,.0f}/{len(h_ID_list):,.0f} ({math.floor((i+1)/(len(h_ID_list))*1e2)/1e2:.2f})".replace(",", "%TEMP%").replace(".", ",").replace("%TEMP%", "."))
        
        log.write(f"Downloading {h_ID_list[i]}...")
        try:
            title, pages=download_hentai(h_ID_list[i])  #download hentai and save images, returns number of pages and title in hentai
        except FileExistsError: #PDF already exists, don't download and convert, skip
            log.write(f"\r{h_ID_list[i]} has already been downloaded and converted. Skipped.")
            i+=1
            i_changed=True
            continue
        except FileNotFoundError:   #gallery got deleted and returned error 404, don't download and convert, skip
            log.write(f"\rnHentai returned error 404 for {h_ID_list[i]}. Skipped.")
            i+=1
            i_changed=True
            continue
        
        log.write(f"Converting {h_ID_list[i]} to PDF...")
        if 10<len(h_ID_list):                       #if more than 10 hentais desired: save in extra folder
            os.makedirs(f"hentai", exist_ok=True)
        if convert_jpg_to_pdf(h_ID_list[i], title, pages)==False:   #convert and merge images to pdf, return bool indicates success
            i_changed=False
            continue    #if converting unsuccessful: corrupted image somewhere, retrying download
        log.write(f"\rConverted and saved {h_ID_list[i]} as PDF.")
        
        try:
            shutil.rmtree(f"./{h_ID_list[i]}/") #remove temp .jpg folder
        except FileNotFoundError:
            pass
        except PermissionError:                 #if impossible: leave behind for later
            pass
        
        i+=1
        i_changed=True

    log.write("Waiting 5s...")
    time.sleep(5)
    log.write("Removing all remaining temporary folders...")
    for h_ID in h_ID_list:  #work through all desired hentai
        try:
            shutil.rmtree(f"./{h_ID}/") #if left behind: retry to remove temp .jpg folder
        except(FileNotFoundError, PermissionError):
            pass
    try:
        shutil.rmtree(f"./__pycache__/") #remove ./__pycache__/
    except(FileNotFoundError, PermissionError):
        pass

    log.write("Press enter to close program.")
    input() #pause
    return  #close program