import concurrent.futures
import lxml.html    #HTML parsing
import os           #image folder
import requests
import time
from all_threads_done import all_threads_done
from download_page import download_page
import KFS.log, KFS.typecheck


def download_hentai(h_ID: int, cookies: dict, HEADERS: dict, MULTITHREADING: bool) -> tuple((str, int)):
    force_loop_entry=True   #force to enter loop? necessary because number of pages may become 0 if HTML parsing is erroneous and loop will then be left
    gallery=None            #hentai gallery from requests.get
    pages=0                 #number of pages, initialised with invalid number
    pages_downloaded=0      #number of pages currently existing in folder, including already existing pages not downloaded
    title=""                #hentai title
    threads=[]              #worker threads, downloads 1 image each


    KFS.typecheck.check(download_hentai, locals(), KFS.typecheck.Mode.instance, KFS.typecheck.Mode.strict, KFS.typecheck.Mode.strict, KFS.typecheck.Mode.strict)


    while pages_downloaded<pages or force_loop_entry==True: #if not all images could be downloaded in first round: retry with new threadpool, new connection
        force_loop_entry=False  #fall back to false after entering
        
        try:
            gallery=requests.get(f'https://nhentai.net/g/{h_ID}/', cookies=cookies, headers=HEADERS, timeout=5) #download gallery
        except(requests.exceptions.ConnectionError, requests.exceptions.ReadTimeout):
            force_loop_entry=True
            time.sleep(5)
            continue
        if gallery.status_code==404:    #gallery got deleted
            raise FileNotFoundError 
        if gallery.status_code==503:    #cloudfare protection kicked: try again
            force_loop_entry=True
            time.sleep(1)
            continue
        gallery=lxml.html.fromstring(gallery.text)   #parse
        
        pages=int(len(gallery.xpath('//div[@class="thumb-container"]')))    #number of pages
        if pages<=0:
            force_loop_entry=True   #get number of pages again
            continue
        try:
            title=str(gallery.xpath('//div[@id="info"]/h1/span[@class="pretty"]/text()')[0])    #title
        except IndexError:
            force_loop_entry=True
            continue
        
        title=title.replace("\\", "")   #remove forbidden characters for filenames
        title=title.replace("/", "")
        title=title.replace(":", "")
        title=title.replace("*", "")
        title=title.replace("?", "")
        title=title.replace("\"", "")
        title=title.replace("<", "")
        title=title.replace(">", "")
        title=title.replace("|", "")
        title=title.replace("\t", "")
        title=title.replace("\n", "")
        title=title[:140]               #limit title length to 140 characters
        if os.path.isfile(f"./{h_ID} {title}.pdf")==True or os.path.isfile(f"./hentai/{h_ID} {title}.pdf")==True:   #PDF already exists, don't download and convert, skip
            raise FileExistsError
        
        os.makedirs(f"{h_ID}", exist_ok=True)   #create image folder ./h_ID/


        with concurrent.futures.ThreadPoolExecutor() as thread_manager:     #download
            for page_nr in range(1, pages+1):                               #download missing pages, save in image folder
                if os.path.isfile(f"./{h_ID}/{h_ID}-{page_nr}.jpg")==True:  #if image already exists: skip
                    continue
                
                if MULTITHREADING==True:
                    threads.append(thread_manager.submit(download_page, h_ID, page_nr, cookies, HEADERS))   #download and save page in worker thread
                elif MULTITHREADING==False:
                    download_page(h_ID, page_nr, cookies, HEADERS)
                    KFS.log.write(f"\rDownloaded {h_ID} page {page_nr:,.0f}/{pages:,.0f}.".replace(",", "."))   #refresh pages downloaded counter
                else:
                    raise RuntimeError("Error in download_hentai(...): Invalid MULTITHREADING value.")

            while all_threads_done(threads)==False:                                                                             #progess display in multithreaded mode, as long as threads still running:
                pages_downloaded_new=len([entry for entry in os.listdir(f"./{h_ID}/") if os.path.isfile(f"./{h_ID}/{entry}")])  #pages already downloaded by counting image files
                if(pages_downloaded==pages_downloaded_new):                                                                     #if number hasn't changed: don't write on console
                    continue
                
                pages_downloaded=pages_downloaded_new   #refresh pages downloaded counter
                KFS.log.write(f"\rDownloaded {h_ID} page {pages_downloaded:,.0f}/{pages:,.0f}.".replace(",", "."))
            pages_downloaded=len([entry for entry in os.listdir(f"./{h_ID}/") if os.path.isfile(f"./{h_ID}/{entry}")])  #refresh pages downloaded counter one last time after threads are finished and in case of everything already downloaded progress display loop will not be executed, to leave outer loop pages_downloaded needs initial value
            KFS.log.write(f"\rDownloaded {h_ID} page {pages_downloaded:,.0f}/{pages:,.0f}.".replace(",", "."))

    return title, pages