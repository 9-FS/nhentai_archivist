# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
from KFSlog import KFSlog
import logging
import multiprocessing
import traceback
from main import main


if __name__=="__main__":
    multiprocessing.freeze_support()    # for multiprocessing to work on windows executables
    KFSlog.setup_logging("", logging.INFO)
    #KFSlog.setup_logging("", logging.DEBUG, filepath_format="./log/%Y-%m-%dT%H_%M.log", rotate_filepath_when="M")

    try:
        main()
    except:
        logging.critical(traceback.format_exc())
        print("\nPress enter to close program.", flush=True)
        input() # pause