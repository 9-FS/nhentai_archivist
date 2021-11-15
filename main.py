import datetime as dt
import math
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai import download_hentai
from get_h_ID_list import get_h_ID_list


def main():
    h_ID_list=list()    #hentai ID to download


    DT_now=dt.datetime.now(dt.timezone.utc)
    print("--------------------------------------------------\n")
    print(f"{DT_now.strftime('%Y-%m-%dT%H:%M:%S')} | {math.floor(DT_now.timestamp()):,.0f}\n".replace(",", "."))
    
    h_ID_list=get_h_ID_list()           #get desired hentai ID
    
    for h_ID in h_ID_list:
        print(f"Downloading {h_ID}...", end="", flush=True)
        title, pages=download_hentai(h_ID)      #download hentai and save images, returns number of pages in hentai
        convert_jpg_to_pdf(h_ID, title, pages)  #convert and merge images to pdf
        print("\r                                                                                                    ", end="")
        print(f"\rDownloaded {h_ID}.")
        print("--------------------------------------------------\n")