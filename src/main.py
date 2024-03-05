# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import gc   # garbage collector, explicitly free memory
import json
from KFSconfig import KFSconfig
from KFSfstr   import KFSfstr
from KFSlog    import KFSlog
from KFSmedia  import KFSmedia
import logging
import os
from get_hentai_ID_list               import get_hentai_ID_list
from Hentai                           import Hentai


@KFSlog.timeit
def main(DEBUG: bool):
    cleanup_success: bool=True                          # cleanup successful
    cookies: dict[str, str]                             # for requests.get to bypass bot protection
    COOKIES_DEFAULT: str=json.dumps({                   # cookies default
        "cf_clearance": "",
        "csrftoken": "",
    }, indent=4)
    DOWNLOADME_FILEPATH: str="./config/downloadme.txt"  # path to file containing hentai ID to download
    headers: dict[str, str]                             # for requests.get to bypass bot protection
    HEADERS_DEFAULT: str=json.dumps({                   # headers default
        "User-Agent": "",
    }, indent=4)
    hentai: Hentai                                      # individual hentai
    hentai_ID_list: list[int]                           # hentai ID to download
    settings: dict[str, str]                            # settings
    SETTINGS_DEFAULT: str=json.dumps({                  # settings default
        "library_path": "./hentai/",                    # path to download hentai to
        "library_split": "0",                           # split library into subdirectories of maximum this many hentai, 0 to disable
    }, indent=4)


    try:
        cookies =json.loads(KFSconfig.load_config("./config/cookies.json",  COOKIES_DEFAULT))   # load cookies to bypass bot protection
        headers =json.loads(KFSconfig.load_config("./config/headers.json",  HEADERS_DEFAULT))   # load headers to bypass bot protection
        settings=json.loads(KFSconfig.load_config("./config/settings.json", SETTINGS_DEFAULT))  # load settings
    except FileNotFoundError:
        return
    hentai_ID_list=get_hentai_ID_list(DOWNLOADME_FILEPATH)                                      # get desired hentai ID
    

    for i, hentai_ID in enumerate(hentai_ID_list):  # work through all desired hentai
        logging.info("--------------------------------------------------")
        logging.info(f"{KFSfstr.notation_abs(i+1, 0, round_static=True)}/{KFSfstr.notation_abs(len(hentai_ID_list), 0, round_static=True)} ({KFSfstr.notation_abs((i+1)/(len(hentai_ID_list)), 2, round_static=True)})")

        if (i+1)%100==0:    # save galleries to file, only every 100 hentai to save time
            Hentai.save_galleries()

        try:
            hentai=Hentai(hentai_ID, cookies, headers)  # create hentai object
        except ValueError:                              # if hentai does not exist:
            continue                                    # skip to next hentai
        else:
            logging.info(hentai)

        try:
            _=hentai.download(settings["library_path"], int(settings["library_split"])) # download hentai
        except FileExistsError:                                                         # if hentai already exists:
            continue                                                                    # skip to next hentai
        except KFSmedia.DownloadError:
            with open("./log/FAILURES.txt", "at") as fails_file:                        # append in failure file
                fails_file.write(f"{hentai.ID}\n")
            continue                                                                    # skip to next hentai
        del _
        gc.collect()                                                                    # explicitly free memory, otherwise PDF may clutter memory
    logging.info("--------------------------------------------------")


    Hentai.save_galleries() # save all galleries to file

    logging.info("Deleting leftover image directories...")
    for hentai_ID in hentai_ID_list:                                                                                                                                # attempt final cleanup
        if os.path.isdir(os.path.join(settings["library_path"], str(hentai_ID))) and len(os.listdir(os.path.join(settings["library_path"], str(hentai_ID))))==0:    # if cache folder still exists and is empty:
            try:
                os.rmdir(os.path.join(settings["library_path"], str(hentai_ID)))                                                                                    # try to clean up
            except PermissionError as e:                                                                                                                            # may fail if another process is still using directory like dropbox
                logging.warning(f"Deleting \"{os.path.join(settings["library_path"], str(hentai_ID))}/\" failed with {KFSfstr.full_class_name(e)}.")
                cleanup_success=False                                                                                                                               # cleanup unsuccessful
    if cleanup_success==True:
        logging.info("\rDeleted leftover image directories.")

    return