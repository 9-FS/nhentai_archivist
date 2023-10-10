# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import logging
import os


def remove_hentai_ID_from_downloadme(hentai_ID: int) -> None:
    """
    If "./downlodme.txt" exists, removes hentai_ID from "./downloadme.txt", so it won't be tried to download again.

    Arguments:
    - hentai_ID: hentai ID to remove
    """

    downloadme_content: list[str]   # downloadme.txt lines


    if os.path.isfile("./downloadme.txt")==False:   # if "./downloadme.txt" does not exist: ignore
        return
    

    logging.info(f"Removing hentai ID {hentai_ID} from \"./downloadme.txt\"...")
    with open("./downloadme.txt", "rt") as downloadme_file:
        downloadme_content=downloadme_file.read().split("\n")                                   # read downloadme.txt
    downloadme_content=[line for line in downloadme_content if line.strip(" ")!=str(hentai_ID)] # remove hentai ID from list, keep everything else the same
    with open("./downloadme.txt", "wt") as downloadme_file:
        downloadme_file.write("\n".join(downloadme_content))                                    # overwrite with content updated
    logging.info(f"\rRemoved hentai ID {hentai_ID} from \"./downloadme.txt\".")

    return