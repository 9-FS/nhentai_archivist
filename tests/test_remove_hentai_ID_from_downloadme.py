# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import hypothesis, hypothesis.strategies
import sys

sys.path.append("./")   # enables importing via parent directory
from src.get_hentai_ID_list               import get_hentai_ID_list
from src.remove_hentai_ID_from_downloadme import remove_hentai_ID_from_downloadme


@hypothesis.given(hypothesis.strategies.lists(hypothesis.strategies.integers()|hypothesis.strategies.text()), hypothesis.strategies.integers()) #generate random downloadme.txt containing numbers (valid) and strings (invalid) and random hentai ID to remove
def test_get_hentai_ID_list(downloadme_content: list[int|str], hentai_ID_to_remove: int) -> None:
    downloadme_content_new: list[str]   # downloadme.txt content after removing hentai_ID_to_remove


    with open("./downloadme.txt", "wt", errors="ignore") as downloadme_file:            # save random downloadme.txt containing numbers (valid) and strings (invalid)
        downloadme_file.write("\n".join([str(line) for line in downloadme_content]))    # list[int|str] -> list[str] -> str, write to file

    remove_hentai_ID_from_downloadme(hentai_ID_to_remove) # remove random hentai ID from random downloadme.txt

    with open("./downloadme.txt", "rt", errors="ignore") as downloadme_file:    # load random downloadme.txt containing numbers (valid) and strings (invalid)
        downloadme_content_new=downloadme_file.read().split("\n")               # str -> list[str]


    assert str(hentai_ID_to_remove) not in downloadme_content_new # check if random hentai ID was removed from random downloadme.txt

    return