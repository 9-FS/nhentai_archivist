# Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import gc   # garbage collector, explicitly free memory
from KFSconfig import KFSconfig
from KFSfstr   import KFSfstr
from KFSlog    import KFSlog
from KFSmedia  import KFSmedia
import logging
import os
import typing
from get_hentai_ID_list import get_hentai_ID_list
from Hentai             import Hentai


@KFSlog.timeit()
def main(DEBUG: bool):
    cleanup_success: bool=True                              # cleanup successful
    config: dict[str, typing.Any]                           # config
    CONFIG_DEFAULT: dict[str, typing.Any]=\
    {
        "DOWNLOADME_FILEPATH": "./config/downloadme.txt",   # path to file containing hentai ID to download
        "LIBRARY_PATH": "./hentai/",                        # path to download hentai to
        "LIBRARY_SPLIT": 0,                                 # split library into subdirectories of maximum this many hentai, 0 to disable
    }
    env: dict[str, str]                                     # environment variables
    ENV_DEFAULT: dict[str, str]=\
    {
        "CF_CLEARANCE": "",                                 # for requests.get to bypass bot protection
        "CSRFTOKEN": "",
        "USER_AGENT": "",
    }
    hentai: Hentai                                          # individual hentai
    hentai_ID_list: list[int]                               # hentai ID to download


    try:
        config=KFSconfig.load_config(env=False, config_filepaths=["./config/config.json"], config_default=CONFIG_DEFAULT)   # load configuration
        env   =KFSconfig.load_config(           config_filepaths=["./.env"],               config_default=ENV_DEFAULT)      # load environment variables
    except ValueError:
        return
    hentai_ID_list=get_hentai_ID_list(config["DOWNLOADME_FILEPATH"])                                                        # get desired hentai ID
    

    for i, hentai_ID in enumerate(hentai_ID_list):  # work through all desired hentai
        logging.info("--------------------------------------------------")
        logging.info(f"{KFSfstr.notation_abs(i+1, 0, round_static=True)}/{KFSfstr.notation_abs(len(hentai_ID_list), 0, round_static=True)} ({KFSfstr.notation_abs((i+1)/(len(hentai_ID_list)), 2, round_static=True)})")

        if (i+1)%100==0:    # save galleries to file, only every 100 hentai to save time
            Hentai.save_galleries()

        try:
            hentai=Hentai(hentai_ID, {"cf_clearance": env["CF_CLEARANCE"], "csrftoken": env["CSRFTOKEN"]}, {"User-Agent": env["USER_AGENT"]})   # create hentai object
        except ValueError:                                                                                                                      # if hentai does not exist:
            continue                                                                                                                            # skip to next hentai
        else:
            logging.info(hentai)

        try:
            _=hentai.download(config["LIBRARY_PATH"], config["LIBRARY_SPLIT"])  # download hentai
        except FileExistsError:                                                 # if hentai already exists:
            continue                                                            # skip to next hentai
        except KFSmedia.DownloadError:
            with open("./log/FAILURES.txt", "at") as fails_file:                # append in failure file
                fails_file.write(f"{hentai.ID}\n")
            continue                                                            # skip to next hentai
        del _
        gc.collect()                                                            # explicitly free memory, otherwise PDF may clutter memory
    logging.info("--------------------------------------------------")


    Hentai.save_galleries() # save all galleries to file

    logging.info("Deleting leftover image directories...")
    for hentai_ID in hentai_ID_list:                                                                                                                            # attempt final cleanup
        if os.path.isdir(os.path.join(config["LIBRARY_PATH"], str(hentai_ID))) and len(os.listdir(os.path.join(config["LIBRARY_PATH"], str(hentai_ID))))==0:    # if cache folder still exists and is empty:
            try:
                os.rmdir(os.path.join(config["LIBRARY_PATH"], str(hentai_ID)))                                                                                  # try to clean up
            except PermissionError as e:                                                                                                                        # may fail if another process is still using directory like dropbox
                logging.warning(f"Deleting \"{os.path.join(config["LIBRARY_PATH"], str(hentai_ID))}/\" failed with {KFSfstr.full_class_name(e)}.")
                cleanup_success=False                                                                                                                           # cleanup unsuccessful
    if cleanup_success==True:
        logging.info("\rDeleted leftover image directories.")

    return