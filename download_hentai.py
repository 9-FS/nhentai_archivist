import concurrent.futures
import datetime as dt
from lxml import html #HTML parsing
import os   #image folder
import requests
from download_page import download_page


def download_hentai(h_ID):
    gallery=None        #hentai gallery from requests.get
    pages=0             #number of pages
    pages_downloaded=0  #number of pages currently existing in folder, including already existing pages not downloaded
    title=""            #hentai title
    threads=list()      #worker threads, downloads 1 image each


    gallery=requests.get(f'https://nhentai.net/g/{h_ID}/')  #download gallery
    gallery=html.fromstring(gallery.text)                   #parse
    
    pages=int(len(gallery.xpath('//div[@class="thumb-container"]')))                    #number of pages
    title=str(gallery.xpath('//div[@id="info"]/h1/span[@class="pretty"]/text()')[0])    #title
    title=title.replace("\\", "")                                                       #remove forbidden characters for filenames
    title=title.replace("/", "")
    title=title.replace(":", "")
    title=title.replace("*", "")
    title=title.replace("?", "")
    title=title.replace("\"", "")
    title=title.replace("<", "")
    title=title.replace(">", "")
    title=title.replace("|", "")
    os.makedirs(f"{h_ID}", exist_ok=True)   #create image folder ./h_ID/


    with concurrent.futures.ThreadPoolExecutor() as thread_manager:     #download
        for page_nr in range(1, pages+1):                               #download missing pages, save in image folder
            if os.path.isfile(f"./{h_ID}/{h_ID}-{page_nr}.jpg")==True:  #if image already exists: skip
                continue

            threads.append(thread_manager.submit(download_page, h_ID, page_nr)) #download and save page in worker thread
            #download_page(h_ID, page_nr)

        while pages_downloaded<pages:   #progess display, as long as pages still need to be downloaded:
            pages_downloaded_new=len([entry for entry in os.listdir(f"./{h_ID}/") if os.path.isfile(f"./{h_ID}/{entry}")])  #pages already downloaded by counting image files
            if(pages_downloaded==pages_downloaded_new): #if number hasn't changed: don't write on console
                continue
            
            pages_downloaded=pages_downloaded_new   #refresh pages downloaded counter
            print("\r                                                                                                    ", end="")
            print(f"\rDownloaded {h_ID} page {pages_downloaded}/{pages}.", end="", flush=True)

    return title, pages