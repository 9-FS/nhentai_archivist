import datetime as dt
import math
import shutil
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai import download_hentai
from get_h_ID_list import get_h_ID_list


def main():
    h_ID_list=list()    #hentai ID to download


    DT_now=dt.datetime.now(dt.timezone.utc)
    print("--------------------------------------------------")
    print(f"{DT_now.strftime('%Y-%m-%dT%H:%M:%S')} | {math.floor(DT_now.timestamp()):,.0f}\n".replace(",", "."))
    
    h_ID_list=get_h_ID_list()                   #get desired hentai ID
    
    i=0
    while i<len(h_ID_list): #work through all desired hentai
        print(f"Downloading {h_ID_list[i]}...", end="", flush=True)
        title, pages=download_hentai(h_ID_list[i])  #download hentai and save images, returns number of pages and title in hentai
        print("\r                                                                                                    ", end="")
        print(f"\rDownloaded {h_ID_list[i]}.")
        
        print(f"Converting {h_ID_list[i]} to PDF...", end="", flush=True)
        if convert_jpg_to_pdf(h_ID_list[i], title, pages)==False:   #convert and merge images to pdf, return bool indicates success
            print("")
            continue    #if converting unsuccessful: corrupted image somewhere, retrying download
        print("\r                                                                                                    ", end="")
        print(f"\rConverted and saved {h_ID_list[i]} as PDF.")
        
        try:
            shutil.rmtree(f"./{h_ID_list[i]}/") #remove temp .jpg folder
        except PermissionError:                 #if impossible: leave behind
            pass
        
        print("--------------------------------------------------")
        i+=1