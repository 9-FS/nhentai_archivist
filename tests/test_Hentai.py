# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import datetime as dt
import hypothesis, hypothesis.strategies
import PIL.Image
import json
from KFSmedia import KFSmedia
import os
import random
import requests
import sys

sys.path.append("./")   # enables importing via parent directory
from src.Hentai import Hentai


@hypothesis.settings(deadline=dt.timedelta(seconds=10))  # increase deadline for individual test because of web requests
@hypothesis.given(hypothesis.strategies.integers())
def test___init__(hentai_ID: int) -> None:
    bypass_bot_protection: bool=True if 0.05<random.random() else False # randomly choose to not bypass bot protection sometimes
    cookies: dict[str, str]={}                                          # for requests.get to bypass bot protection
    headers: dict[str, str]={}                                          # for requests.get to bypass bot protection


    if bypass_bot_protection==True:
        with open("./cookies.json", "rt") as cookies_file:  # load cookies to bypass bot protection
            cookies=json.loads(cookies_file.read())
        with open("./headers.json", "rt") as headers_file:  # load headers to bypass bot protection
            headers=json.loads(headers_file.read())


    try:
        hentai=Hentai(hentai_ID, cookies, headers)  # create hentai object with cookies and headers
    except ValueError:                              # if input discarded as invalid: check if input really invalid
        # TODO how to check independently from the way used in __init__(...)?
        return
    except requests.HTTPError:              # if failed to bypass bot protection:
        assert bypass_bot_protection==False # check that bot protection was not supposed to be bypassed
        return
    else:
        assert 1<=hentai_ID                 # check that hentai ID has a chance to be valid
        assert bypass_bot_protection==True  # check that bot protection was supposed to be bypassed

    assert isinstance(hentai._gallery, dict)                    # check that hentai._gallery is a dict
    assert "images" in hentai._gallery                          # check that hentai._gallery["images"] exists
    assert "pages" in hentai._gallery["images"]                 # check that hentai._gallery["images"]["pages"] exists
    assert isinstance(hentai._gallery["images"]["pages"], list) # check that hentai._gallery["images"]["pages"] is a list
    for page in hentai._gallery["images"]["pages"]:
        assert "t" in page                                      # check that hentai._gallery["images"]["pages"][page]["t"] exists

    assert hentai.ID==hentai_ID  # check that hentai ID is correct

    assert isinstance(hentai.page_amount, int)  # check that hentai.page_amount is an int
    assert 0<hentai.page_amount                 # check that hentai.page_amount is positive

    assert isinstance(hentai.title, str)  # check that hentai.title is a str

    assert isinstance(hentai._fails, list)          # check that hentai._fails is a list
    assert len(hentai._fails)==hentai.page_amount   # check that hentai._fails has same length as hentai.page_amount
    for fail in hentai._fails:                      # check that hentai._fails is a list initialised with 0
        assert isinstance(fail, int)                # check that hentai._fails is a list of ints
        assert fail==0
    
    return


@hypothesis.settings(deadline=dt.timedelta(seconds=100))    # increase deadline for individual test because of web requests
@hypothesis.given(hypothesis.strategies.integers())
def test_download(hentai_ID: int) -> None:
    cookies: dict[str, str]     # for requests.get to bypass bot protection
    headers: dict[str, str]     # for requests.get to bypass bot protection
    PDF: list[PIL.Image.Image]  # PFD downloaded
    settings: dict[str, str]    # settings


    with open("./cookies.json", "rt") as cookies_file:      # load cookies to bypass bot protection
        cookies=json.loads(cookies_file.read())
    with open("./headers.json", "rt") as headers_file:      # load headers to bypass bot protection
        headers=json.loads(headers_file.read())
    with open("./settings.json", "rt") as settings_file:    # load settings
        settings=json.loads(settings_file.read())

    try:
        hentai=Hentai(hentai_ID, cookies, headers)  # create hentai object with cookies and headers
    except ValueError:                              # if input discarded as invalid: 
        return


    try:
        PDF=hentai.download(settings["dest_path"])      # download hentai
    except FileExistsError:                             # if hentai already exists:
        assert isinstance(hentai.PDF_filepath, str)     # check that hentai.PDF_filepath is a str
        assert os.path.isfile(hentai.PDF_filepath)      # check that hentai.PDF_filepath is a file
        return
    except KFSmedia.DownloadError:
        return
    
    assert isinstance(hentai.PDF_filepath, str)     # check that hentai.PDF_filepath is a str
    assert os.path.isfile(hentai.PDF_filepath)      # check that hentai.PDF_filepath is a file
    assert len(PDF)==hentai.page_amount             # check that hentai.PDF_filepath has same length as hentai.page_amount

    return