import datetime as dt
import math
import shutil   #remove tree
import time
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai import download_hentai
from get_h_ID_list import get_h_ID_list


def main():
    h_ID_list=[]    #hentai ID to download


    DT_now=dt.datetime.now(dt.timezone.utc)
    print("--------------------------------------------------")
    print(f"{DT_now.strftime('%Y-%m-%dT%H:%M:%S')} | {math.floor(DT_now.timestamp()):,.0f}\n".replace(",", "."))
    
    h_ID_list=get_h_ID_list()                   #get desired hentai ID
    
    i=0
    while i<len(h_ID_list): #work through all desired hentai
        print(f"{i+1}/{len(h_ID_list)}")
        print(f"Downloading {h_ID_list[i]}...", end="", flush=True)
        try:
            title, pages=download_hentai(h_ID_list[i])  #download hentai and save images, returns number of pages and title in hentai
        except FileExistsError: #PDF already exists, don't download and convert, skip
            print("\r                                                                                                    ", end="")
            print(f"\r{h_ID_list[i]} has already been downloaded and converted. Skipped.")
            print("--------------------------------------------------")
            i+=1
            continue
        print("")
        
        print(f"Converting {h_ID_list[i]} to PDF...", end="", flush=True)
        if convert_jpg_to_pdf(h_ID_list[i], title, pages)==False:   #convert and merge images to pdf, return bool indicates success
            print("")
            continue    #if converting unsuccessful: corrupted image somewhere, retrying download
        print("\r                                                                                                    ", end="")
        print(f"\rConverted and saved {h_ID_list[i]} as PDF.")
        
        try:
            shutil.rmtree(f"./{h_ID_list[i]}/") #remove temp .jpg folder
        except FileNotFoundError:
            pass
        except PermissionError:                 #if impossible: leave behind for later
            pass
        
        print("--------------------------------------------------")
        i+=1

    print("Waiting 5s...")
    time.sleep(5)
    print("Removing all remaining temporary folders...")
    for h_ID in h_ID_list:  #work through all desired hentai
        try:
            shutil.rmtree(f"./{h_ID}/") #if left behind: retry to remove temp .jpg folder
        except FileNotFoundError:
            pass
        except PermissionError:         #if impossible: leave behind
            pass
    try:
        shutil.rmtree(f"./__pycache__/") #remove ./__pycache__/
    except FileNotFoundError:
        pass
    except PermissionError:         #if impossible: leave behind
        pass