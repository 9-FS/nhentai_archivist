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
    hentai_ID_list: list[int]=[]    # hentai ID list to download

    
    while True:
        if os.path.isfile("./downloadme.txt")==True and file_tried==False:                                      # if ID list in file and not tried to load from file yet: load from file, only try once
            file_tried=True
            with open("downloadme.txt", "rt") as downloadme_file:
                hentai_ID_list=_convert_hentai_ID_list_str_to_hentai_ID_list_int(downloadme_file.readlines())   # read all hentai ID from file, list[int] -> list[str]
        else:                                                                                                   # if ID list file not available: ask user for input
            logging.info("Enter the holy numbers: ")
            hentai_ID_list=_convert_hentai_ID_list_str_to_hentai_ID_list_int(input().split(" "))                # user input seperated at whitespace, list[int] -> list[str]
        
        if len(hentai_ID_list)==0:  # if file or user input empty: retry
            continue

        break

    return hentai_ID_list


def _convert_hentai_ID_list_str_to_hentai_ID_list_int(hentai_ID_list_str: list[str]) -> list[int]:
    """
    Converts list of hentai ID from list[str] to list[int] cleans up entries.

    Arguments:
    - hentai_ID_list_str: list of hentai ID in str to convert

    Returns:
    - hentai_ID_list: list of hentai ID in int
    """

    hentai_ID_list: list[int]=[]    # list of hentai ID in int


    hentai_ID_list_str=[hentai_ID for hentai_ID in hentai_ID_list_str if len(hentai_ID)!=0] # throw out emtpy entries
    hentai_ID_list_str=list(dict.fromkeys(hentai_ID_list_str))                              # remove duplicates

    for hentai_ID in hentai_ID_list_str:                                                    # list[str] -> list[int]
        try:
            hentai_ID_list.append(int(hentai_ID))
        except ValueError:                                                                  # if input invalid: discard that, keep rest
            logging.error(f"Converting input \"{hentai_ID}\" to int failed. Skipping ID.")
            
    hentai_ID_list=sorted(hentai_ID_list)                                                   # sort numerically

    return hentai_ID_list