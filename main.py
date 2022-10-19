import math
import os
import PIL
import shutil   #remove tree
import time
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai    import download_hentai
from get_h_ID_list      import get_h_ID_list
import KFS.log


def main():
    h_ID_list=[]        #hentai ID to download
    conversion_fails=[] #for every page in hentai how many times conversion failed? only allow 10 times before giving up on hentai


    KFS.log.write("--------------------------------------------------")
    h_ID_list=get_h_ID_list()   #get desired hentai ID
    if 10<len(h_ID_list):       #if more than 10 hentais desired: save in extra folder
        os.makedirs("./hentai/", exist_ok=True)
    
    
    i=0
    i_changed=True  #i changed since last iteration, for console printouts -------
    while i<len(h_ID_list): #work through all desired hentai
        if i_changed==True:
            KFS.log.write("--------------------------------------------------")
            KFS.log.write(f"{i+1:,.0f}/{len(h_ID_list):,.0f} ({math.floor((i+1)/(len(h_ID_list))*1e2)/1e2:.2f})".replace(",", "%TEMP%").replace(".", ",").replace("%TEMP%", "."))
        
        KFS.log.write(f"Downloading {h_ID_list[i]}...")
        try:
            title, pages=download_hentai(h_ID_list[i])  #download hentai and save images, returns number of pages and title in hentai
        except FileExistsError: #PDF already exists, don't download and convert, skip
            KFS.log.write(f"\r{h_ID_list[i]} has already been downloaded and converted. Skipped.")
            i+=1
            i_changed=True
            continue
        except FileNotFoundError:   #gallery got deleted and returned error 404, don't download and convert, skip
            KFS.log.write(f"\rnHentai returned error 404 for {h_ID_list[i]}. Skipped.")
            i+=1
            i_changed=True
            continue

        if i_changed==True:     #since page number now known, if first iteration: initialise conversion fails list
            conversion_fails=[] #reset first
            for j in range(pages):
                conversion_fails.append(0)
        

        KFS.log.write(f"Converting {h_ID_list[i]} to PDF...")
        try:
            convert_jpg_to_pdf(h_ID_list[i], title, pages, conversion_fails)    #convert and merge images to pdf
        except (FileNotFoundError, PIL.UnidentifiedImageError):                 #if converting unsuccessful: corrupted image somewhere, retrying download
            i_changed=False
            continue    
        except (PermissionError, RuntimeError):         #corrupt image could not be converted or deleted after 10 times, giving hentai up, not cleaning up
            with open("Fails.txt", "at") as fails_file: #save fails in file
                fails_file.write(f"{h_ID_list[i]}\n")
            i+=1
            i_changed=True
            continue
        KFS.log.write(f"\rConverted and saved {h_ID_list[i]} as PDF.")
        
        try:
            shutil.rmtree(f"./{h_ID_list[i]}/")         #remove temp .jpg folder
        except (FileNotFoundError, PermissionError):    #if impossible: leave behind for later
            pass
        
        i+=1
        i_changed=True


    KFS.log.write("Waiting 5s...")
    time.sleep(5)
    KFS.log.write("Removing all remaining temporary folders...")
    for h_ID in h_ID_list:  #work through all desired hentai
        try:
            shutil.rmtree(f"./{h_ID}/") #if left behind: retry to remove temp .jpg folder
        except (FileNotFoundError, PermissionError):
            pass
    try:
        shutil.rmtree(f"./__pycache__/") #remove ./__pycache__/
    except (FileNotFoundError, PermissionError):
        pass

    KFS.log.write("Press enter to close program.")
    input() #pause
    return  #close program