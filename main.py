import json
import math
import os
import PIL
import shutil   #remove tree
import time
from convert_jpg_to_pdf import convert_jpg_to_pdf
from download_hentai    import download_hentai
from get_h_ID_list      import get_h_ID_list
import KFS.log


@KFS.log.timeit
def main():
    cookies=""              #for requests.get to bypass cloudflare
    conversion_fails=[]     #for every page in hentai how many times conversion failed? only allow 10 times before giving up on hentai
    h_ID_list=[]            #hentai ID to download
    HEADERS={
        'authority': 'nhentai.net',
        'accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9',
        'accept-language': 'en-US,en;q=0.9',
        'cache-control': 'max-age=0', 
        'referer': 'https://nhentai.net/',
        'sec-ch-ua': '"Chromium";v="106", "Google Chrome";v="106", "Not;A=Brand";v="99"',
        'sec-ch-ua-mobile': '?0',
        'sec-ch-ua-platform': '"Windows"',
        'sec-fetch-dest': 'document',
        'sec-fetch-mode': 'navigate',
        'sec-fetch-site': 'same-origin',
        'sec-fetch-user': '?1',
        'upgrade-insecure-requests': '1',
        'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36'
    }                       #for requests.get to bypass clouflare
    MULTITHREADING=True    #use multithreading to download several pages at once?


    KFS.log.write("--------------------------------------------------")
    with open("cookies.json") as cookies_file:  #load cookies
        cookies=json.loads(cookies_file.read())
    h_ID_list=get_h_ID_list()                   #get desired hentai ID
    if 10<len(h_ID_list):                       #if more than 10 hentais desired: save in extra folder
        os.makedirs("./hentai/", exist_ok=True)
    
    
    i=0
    i_changed=True          #i changed since last iteration, for console printouts -------
    while i<len(h_ID_list): #work through all desired hentai
        if i_changed==True:
            KFS.log.write("--------------------------------------------------")
            KFS.log.write(f"{i+1:,.0f}/{len(h_ID_list):,.0f} ({math.floor((i+1)/(len(h_ID_list))*1e2)/1e2:.2f})".replace(",", "%TEMP%").replace(".", ",").replace("%TEMP%", "."))
        
        KFS.log.write(f"Downloading {h_ID_list[i]}...")
        try:
            title, pages=download_hentai(h_ID_list[i], cookies, HEADERS, MULTITHREADING)    #download hentai and save images, returns number of pages and title in hentai
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

    return  #close program


#bypass cloudflare: https://github.com/Charlzk05/NHentai-Downloader-2022/blob/main/main.py