# Copyright (c) 2023 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
import hypothesis, hypothesis.strategies
import sys
import unittest.mock    # overwrite input()

sys.path.append("./")   # enables importing via parent directory
from src.get_hentai_ID_list import get_hentai_ID_list


@unittest.mock.patch("builtins.input", lambda: "1")                                                             # overwrite input() with valid input, means user is bothered until finally he enters a valid input
@hypothesis.given(hypothesis.strategies.lists(hypothesis.strategies.integers()|hypothesis.strategies.text()))   #generate random downloadme.txt containing numbers (valid) and strings (invalid)
def test_get_hentai_ID_list(downloadme_content: list[int|str]) -> None:
    hentai_ID_list: list[int]   # hentai ID list to download	


    with open("./downloadme.txt", "wt", errors="ignore") as downloadme_file:            # save random downloadme.txt containing numbers (valid) and strings (invalid)
        downloadme_file.write("\n".join([str(line) for line in downloadme_content]))    # list[int|str] -> list[str] -> std, write to file

    hentai_ID_list=get_hentai_ID_list() # get hentai ID list from random downloadme.txt, load from downloadme.txt first, then if that is completely invalid and discarded correctly, use correct user input
    

    assert 0<len(hentai_ID_list)                    # check that hentai ID list is not empty

    for hentai_ID in hentai_ID_list:                # check that hentai ID list does not contain duplicates
        assert hentai_ID_list.count(hentai_ID)==1

    assert sorted(hentai_ID_list)==hentai_ID_list   # check that hentai ID list is sorted

    for hentai_ID in hentai_ID_list:                # check that hentai ID list only contains integers
        assert isinstance(hentai_ID, int)

    for line in downloadme_content:                 # check that hentai ID list contains all valid integers from random downloadme.txt
        try:
            int(line)
        except ValueError:
            continue
        else:
            assert int(line) in hentai_ID_list      
    
    return