# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import logging
import os


def get_hentai_ID_list() -> list[int]:
    """
    Tries to return hentai ID list to download by trying to load "./downloadme.txt" or getting hentai ID by user input.

    Returns:
    - hentai_ID_list: list of hentai ID to download
    """

    file_tried: bool=False          # tried to load from file?
    hentai_ID_list: list[int]=[]    # hentai ID list
    hentai_ID_list_str: list[str]

    
    while True:
        if os.path.isfile("./downloadme.txt")==True and file_tried==False:  # if ID list in file and not tried to load from file yet: load from file, only try once
            file_tried=True
            with open("downloadme.txt", "rt") as downloadme_file:
                hentai_ID_list_str=downloadme_file.readlines()              # read all lines from file
        else:                                                               # if ID list file not available: ask user for input
            logging.info("Enter the holy numbers: ")
            hentai_ID_list_str=input().split(" ")                           # user input seperated at whitespace
        
        hentai_ID_list_str=[hentai_ID for hentai_ID in hentai_ID_list_str if len(hentai_ID)!=0] # remove empty inputs
        if len(hentai_ID_list_str)==0:                                                          # if file or user input empty: retry
            continue

        for hentai_ID in hentai_ID_list_str:    # convert all hentai ID to int
            try:
                hentai_ID_list.append(int(hentai_ID))
            except ValueError:                  # if input invalid: discard whole input, ask user (again)
                logging.error(f"Converting input \"{hentai_ID}\" to int failed.")
                break
        else:                                   # if all ID converted without failure: break out of while, return desired ID
            break

    return hentai_ID_list