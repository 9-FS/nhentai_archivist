from lxml import html   #HTML parsing
from PIL import Image   #jpg to pdf
import requests
import time


def download_page(h_ID, page_nr):
    image=""

    while(len(image)==0):  #repeat if no image because access denied (rate limit)
        try:
            page=requests.get(f'https://nhentai.net/g/{h_ID}/{page_nr}/', timeout=5)    #page gallery
        except TimeoutError:    #if timeout: try again
            continue
        page=html.fromstring(page.text) 
        
        img_link=page.xpath('//section[@id="image-container"]/a/img/@src')  #parse direct image link
        try:
            image=requests.get(img_link[0], timeout=5).content  #download image
        except TimeoutError:    #if timeout: try again
            continue

        time.sleep(1)
        

    with open(f"./{h_ID}/{h_ID}-{page_nr}.jpg", "wb") as img_file:
        img_file.write(image)