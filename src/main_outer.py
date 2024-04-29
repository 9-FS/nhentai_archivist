# Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
from KFSlog import KFSlog
import logging
import multiprocessing
import traceback
from main import main


if __name__=="__main__":
    DEBUG: bool=False   # debug mode?


    if DEBUG==True:
        KFSlog.setup_logging("", logging.DEBUG, filepath_format="./log/%Y-%m-%dT%H_%M.log", rotate_filepath_when="M")
    else:
        KFSlog.setup_logging("", logging.INFO)
    multiprocessing.freeze_support()    # for multiprocessing to work on windows executables


    try:
        main(DEBUG)
    except:
        logging.critical(traceback.format_exc())
        print("\nPress enter to close program.", flush=True)
        input() # pause