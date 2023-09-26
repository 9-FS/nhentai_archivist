# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import inspect
import dataclasses
from KFSmedia import KFSmedia
import json
import logging
import os
import random
import re
import requests
import time


@dataclasses.dataclass
class Hentai:
    """
    represents an individual hentai
    """                    

    def __init__(self, nhentai_ID: int, cookies: dict[str, str], headers: dict[str, str]):
        """
        Constructs hentai object. Downloads data from the nhentai API.

        Arguments:
        - nhentai_ID: the hentai from nhentai.net found here: https://nhentai.net/g/{nhentai_ID}
        - cookies: cookies to send with the request to bypass bot protection
        - headers: user agent to send with the request to bypass bot protection

        Raises:
        - requests.HTTPError: Downloading gallery from \"{NHENTAI_GALLERY_API_URL}/{self.ID}\" failed multiple times.
        - ValueError: Hentai with ID \"{self.ID}\" does not exist.
        """

        gallery_page: requests.Response
        NHENTAI_GALLERY_API_URL: str="https://nhentai.net/api/gallery"  # URL to nhentai API
        self._fails: list[int]                                          # list of how many times individual page has failed to be downloaded or converted to PDF
        self._gallery: dict                                             # gallery from nhentai API, saved to extract data for download later
        self._give_up: bool=False                                       # give this hentai up? after failing to download or convert numerous times
        self.ID: int                                                    # nhentai ID
        self.page_amount: int                                           # number of pages
        self.title: str                                                 # title (unchanged)
        

        logging.debug(f"Creating hentai object...")

        self.ID=nhentai_ID

        logging.info(f"Downloading gallery from \"{NHENTAI_GALLERY_API_URL}/{self.ID}\"...")
        attempt_no: int=1
        while True:
            try:
                gallery_page=requests.get(f"{NHENTAI_GALLERY_API_URL}/{self.ID}", cookies=cookies, headers=headers, timeout=10)
            except (requests.exceptions.ConnectionError, requests.exceptions.ReadTimeout):  # if connection error: try again
                time.sleep(1)
                if attempt_no<3:                                                            # try 3 times
                    continue
                else:                                                                       # if failed 3 times: give up
                    raise
            if gallery_page.status_code==403:                                               # if status code 403 (forbidden): probably cookies and headers not set correctly
                logging.error(f"Downloading gallery from \"{NHENTAI_GALLERY_API_URL}/{self.ID}\" resulted in status code {gallery_page.status_code}. Have you set \"cookies.json\" and \"headers.json\" correctly?")
                raise requests.HTTPError(f"Error in {self.__init__.__name__}{inspect.signature(self.__init__)}: Downloading gallery from \"{NHENTAI_GALLERY_API_URL}/{self.ID}\" resulted in status code {gallery_page.status_code}. Have you set \"cookies.json\" and \"headers.json\" correctly?")
            if gallery_page.status_code==404:                                               # if status code 404 (not found): hentai does not exist (anymore?)
                logging.error(f"Hentai with ID \"{self.ID}\" does not exist.")
                raise ValueError(f"Error in {self.__init__.__name__}{inspect.signature(self.__init__)}: Hentai with ID \"{self.ID}\" does not exist.")
            if gallery_page.ok==False:                                                      # if status code not ok: try again
                time.sleep(1)
                if attempt_no<3:                                                            # try 3 times
                    continue
                else:                                                                       # if failed 3 times: give up
                    raise

            self._gallery=json.loads(gallery_page.text)
            break
        logging.info(f"\rDownloaded gallery from \"{NHENTAI_GALLERY_API_URL}/{self.ID}\".")

        self.page_amount=int(self._gallery["num_pages"])

        self.title=self._gallery["title"]["pretty"]
        
        self._fails=[]
        for _ in range(self.page_amount):   # initialise with amount of pages number of zeros
            self._fails.append(0)

        logging.debug(f"\rCreated hentai object.")
        logging.debug(self.__repr__())
        return
    

    def __str__(self) -> str:
        return f"{self.ID}: \"{self.title}\""
    

    def _increment_fails(self, image_list: list[str]) -> None:
        """
        Takes list of filepaths that could not be downloaded or converted and increments appropiate failure counter.
        """

        PATTERNS: list[str]=[
            r"^((?P<page_no>[0-9]+)\.(jpg|png))$",          # page URL pattern
            r"^([0-9]+-(?P<page_no>[0-9]+)\.(jpg|png))$",   # image filepath pattern
        ]
        re_match: re.Match|None


        for image in image_list:                                                        # for each image:
            for pattern in PATTERNS:                                                    # with each pattern:
                re_match=re.search(pattern, image.split("/")[-1])                       # try to parse page number, use only filename not path
                if re_match!=None:                                                      # if page number could be parsed:
                    self._fails[int(re_match.groupdict()["page_no"])-1]+=1              # increment appropiate fails counter
                    if 10<=self._fails[int(re_match.groupdict()["page_no"])-1]:         # if any counter 10 or above:
                        self._give_up=True                                              # give hentai up
                    break
            else:                                                                       # if page number can't be parsed:
                logging.critical(f"Incrementing fails counter of \"{image}\" failed.")  # don't know which counter to increment, critical error because should not happen
                raise RuntimeError(f"Error in {self._increment_fails.__name__}{inspect.signature(self._increment_fails)}: Incrementing fails counter of \"{image}\" failed.")

        return
    

    def download(self) -> None:
        """
        Downloads the hentai and saves it at f"./{self.ID} {self.title}.pdf".

        Raises:
        - FileExistsError: File \"{PDF_filepath}\" already exists.
        - Hentai.DownloadError:  \"{PDF_filepath}\" already exists as directory or tried to convert hentai \"{self}\" several times, but failed.
        """

        images_filepath: list[str]=[]                       # where to cache downloaded images
        pages_URL: list[str]=[]                             # URL to individual pages to download
        PDF_filepath: str                                   # where to save downloaded result, ID title pdf, but title maximum 140 characters and without illegal filename characters
        TITLE_CHARACTERS_FORBIDDEN: str="\\/:*?\"<>|\t\n"   # in title forbidden characters


        for i, page in enumerate(self._gallery["images"]["pages"]):
            pages_URL.append(f"https://i{random.choice(['', '2', '3', '5', '7'])}.nhentai.net/galleries/{self._gallery['media_id']}/{i+1}") # general URL, use random image server instance to distribute load
            images_filepath.append(f"./hentai/{self.ID}/{self.ID}-{i+1}")
            match page["t"]:                # image type
                case "p":                   # png
                    pages_URL[-1]+=".png"   # append extension
                    images_filepath[-1]+=".png"
                case "j":                   #jpg
                    pages_URL[-1]+=".jpg"   # append extension
                    images_filepath[-1]+=".jpg"

        PDF_filepath=self.title
        for c in TITLE_CHARACTERS_FORBIDDEN:                                                    # remove forbidden characters for filenames
            PDF_filepath=PDF_filepath.replace(c, "")
        PDF_filepath=PDF_filepath[:140]                                                         # limit title length to 140 characters
        PDF_filepath=f"./hentai/{self.ID} {PDF_filepath}.pdf"
        if os.path.isfile(PDF_filepath)==True:                                                  # if PDF already exists: skip download
            logging.info(f"File \"{PDF_filepath}\" already exists. Skipped download.")
            raise FileExistsError(f"File \"{PDF_filepath}\" already exists. Skipped download.") # raise exception to skip upload in main
        if os.path.isdir(PDF_filepath)==True:                                                   # if PDF already exists as directory: skip download, append to failures
            logging.error(f"\"{PDF_filepath}\" already exists as directory. Skipped download.")
            raise KFSmedia.DownloadError(f"Error in {self.download.__name__}{inspect.signature(self.download)}: \"{PDF_filepath}\" already exists as directory. Skipped download.")


        while self._give_up==False:                                     # while not giving up: try to download and convert
            try:
                KFSmedia.download_medias(pages_URL, images_filepath)    # download images # type:ignore
            except KFSmedia.DownloadError as e:
                self._increment_fails(e.args[0])                        # increment fails, may trigger giving up
                continue
                
            try:
                KFSmedia.convert_images_to_PDF(images_filepath, PDF_filepath)   # convert images to PDF
            except KFSmedia.ConversionError as e:
                self._increment_fails(e.args[0])                                # increment fails, may trigger giving up
                continue
            else:                                                               # if conversion successful:
                self.PDF_filepath=PDF_filepath                                  # save PDF filepath
                break                                                           # break out
        else:                                                                   # if giving up:
            logging.error(f"Tried to download and convert hentai \"{self}\" several times, but failed. Giving up.")
            raise KFSmedia.DownloadError(f"Error in {self.download.__name__}{inspect.signature(self.download)}: Tried to download and convert hentai \"{self}\" several times, but failed. Giving up.")

    
        if os.path.isdir(f"./hentai/{self.ID}") and len(os.listdir(f"./hentai/{self.ID}"))==0:  # if cache folder still exists and is empty:
            try:
                os.rmdir(f"./hentai/{self.ID}")                                                 # try to clean up
            except PermissionError:                                                             # may fail if another process is still using directory like dropbox
                pass                                                                            # don't warn because will be retried in main

        return