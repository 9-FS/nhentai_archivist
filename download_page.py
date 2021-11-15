from lxml import html   #HTML parsing
from PIL import Image   #jpg to pdf
import requests


def download_page(h_ID, page_nr):
    page=requests.get(f'https://nhentai.net/g/{h_ID}/{page_nr}/')
    page=html.fromstring(page.text)
    img_link=page.xpath('//section[@id="image-container"]/a/img/@src')

    with open(f"./{h_ID}/{h_ID}-{page_nr}.jpg", "wb") as img_file:
        img_file.write(requests.get(img_link[0]).content)