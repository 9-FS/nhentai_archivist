# nhentai_archivist

> [!IMPORTANT]
> If you have any questions, please **consult the readme first**.
>
> I spend a lot of effort to keep the readme up-to-date and chances are high that your problem has already been addressed here. If you still require assistance after reading the readme, post it publicly, preferably as a GitHub issue, so others can benefit from the troubleshooting as well.

## Introduction

nHentai Archivist is a tool to download hentai from https://nhentai.net and convert them to CBZ files. From quickly downloading a few hentai specified in the console, downloading a few hundred hentai specified in a downloadme.txt, up to automatically keeping a massive self-hosted library up-to-date by automatically generating a downloadme.txt from a search by tag. (For that use-case it has been optimised to tag the CBZ files in a way that [Komga](https://komga.org/) in [oneshot mode](https://komga.org/docs/guides/oneshots) interprets everything correctly.)

Why CBZ? CBZ is a widespread standard and basically just a ZIP file containing the images and a metadata file. This enables nHentai Archivist to **keep the tags** which wouldn't be possible with PDF as far as I know.

Big thanks go out to [h3nTa1m4st3r_xx69](https://github.com/sam-k0), who helped me with using nHentai's completely undocumented API. Without him this project could not have been reactivated.
I'm happy about anyone who finds my software useful and feedback is also always welcome. Happy downloading~

## Quick Start

1. Execute the program once to create a default `./config/.env`.
1. Execute the program again and enter the nHentai ID you want to download separated by spaces.

## Installation

1. Execute the program once to create a default `./config/.env`.\
    This means that in the directory of the executable, there should now be a directory called "config" containing a file called ".env". You might need to enable seeing hidden files in the file explorer.
1. I recommend setting the `CSRFTOKEN` cookie and `USER_AGENT`. If you start having problems with nHentai's bot protection (error 403), setting these is mandatory. If a `CF_CLEARANCE` cookie is available, it should be set as well.
    - Mozilla Firefox
        1. Go to https://nhentai.net/. Clear the Cloudflare prompt.
        1. Open the developer console with F12.
        1. Go to the tab "Storage". On the left side expand "Cookies". Click on "https://nhentai.net".
        1. Copy the cookie values into `./config/.env`.
        1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `./config/.env`.
    - Google Chrome
        1. Go to https://nhentai.net/. Clear the cloudflare prompt.
        1. Open the developer console with F12.
        1. Go to the tab "Application". On the left side under "Storage", expand "Cookies". Click on "https://nhentai.net".
        1. Copy the cookie values into `./config/.env`.
        1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `./config/.env`.

> [!NOTE]
> If nHentai has "under attack" mode enabled, clearing the Cloudflare prompt and updating `CF_CLEARANCE` seem to be required daily.

## Further Settings
- `CLEANUP_TEMPORARY_FILES`, optional, defaults to `true`

    Setting this to `false` prevents the temporary directory containing the original images from being deleted after the CBZ file has been created. In addition to that it also saves a `ComicBook.xml` in the directory. This can be useful to improve compatibility with third party readers or deduplication software.

- `DONTDOWNLOADME_FILEPATH`, optional, defaults to `None`

    This is the path to the file containing the nHentai ID you explicitly do not want to download, separated by line breaks. It has priority over all input methods. If you want to systematically exclude hentai by tag, use the `-` operator in the tag search instead.

- `DOWNLOADME_FILEPATH`, optional, defaults to `None`

    This is the path to the file containing the nHentai ID you want to download, separated by line breaks. If this file exists, it has priority over tag search and console input.

- `FILENAME_TITLE_TYPE`, optional, defaults to `English`

    Determines which title type to use when naming downloaded hentai files. Available settings:
    - `English`
    - `Japanese`
    - `Pretty`

    If a Japanese or pretty title is not available, the English title will be used instead.

> [!IMPORTANT]
> If `FILENAME_TITLE_TYPE` is changed after hentai files have already been downloaded, existing files will not be renamed. They also will not be detected as already downloaded any more which can lead to duplicates. I therefore recommend to only change this setting before starting a fresh library.

- `LIBRARY_PATH`

    This is the directory temporary images and finished CBZ files are download to. By default, it will download to `./hentai/`.

- `LIBRARY_SPLIT`, optional, defaults to `0`

    Setting this to a value other than 0 splits the library at `LIBRARY_PATH` into sub-directories with a maximum number of `LIBRARY_SPLIT` hentai allowed per sub-directory. It is recommended if the number of hentai in 1 directory starts to affect file explorer performance. This _should_ not affect you if you plan to keep less than 10.000 files in your `LIBRARY_PATH` directory, otherwise the recommended setting is `LIBRARY_SPLIT = 10000`.

- `NHENTAI_TAGS`, optional, defaults to `None` (client mode)

    Setting this will trigger "server mode". If no file at `DOWNLOADME_FILEPATH` is found, it will generate one by searching for the tags specified. After all hentai on the downloadme have been downloaded, it will wait for `SLEEP_INTERVAL` seconds and restart the search. This is useful to keep a self-hosted library up-to-date with the latest releases from the specified tag search. Multiple tags and tag exclusions can be specified and are connected via logical AND. This means results must fullfill all criteria specified.

    `./config/.env` examples:

    ```TOML
    NHENTAI_TAGS = ['language:"english"'] # all english hentai
    NHENTAI_TAGS = ['tag:"big breasts"'] # all hentai with the tag "big breasts"
    NHENTAI_TAGS = ['parody:"kono subarashii sekai ni syukufuku o"'] # all hentai from the anime "Kono Subarashii Sekai ni Syukufuku o"
    NHENTAI_TAGS = ['artist:"shindol"'] # all hentai by Shindol
    NHENTAI_TAGS = ['character:"frieren"'] # all hentai with character "Frieren"
    NHENTAI_TAGS = ['tag:"ffm threesome"', 'tag:"sister"', '-tag:"full censorship"', '-tag:"mind control"'] # all hentai with the tags "ffm threesome" and "sister" but without the tags "full censorship" and "mind control"
    ```

    `docker-compose.yaml` examples:

    ```YAML
    environment:
        - NHENTAI_TAGS: '[language:"english"]' # all english hentai
        - NHENTAI_TAGS: '[tag:"big breasts"]' # all hentai with the tag "big breasts"
        - NHENTAI_TAGS: '[parody:"kono subarashii sekai ni syukufuku o"]' # all hentai from the anime "Kono Subarashii Sekai ni Syukufuku o"
        - NHENTAI_TAGS: '[artist:"shindol"]' # all hentai by Shindol
        - NHENTAI_TAGS: '[character:"frieren"]' # all hentai with character "Frieren"
        - NHENTAI_TAGS: '[tag:"ffm threesome", tag:"sister", -tag:"full censorship", -tag:"mind control"]' # all hentai with the tags "ffm threesome" and "sister" but without the tags "full censorship" and "mind control"
    ```

    Pay attention to copy the format exactly as shown in the examples. That includes the usage of single quotation marks outside and double quotation marks inside. If the format is not being copied exactly, at least searching by tags that contain a space leads to erroneous API responses. More information can be found [here](https://nhentai.net/info/).

    The docker compose method can be especially useful if altering quickly between searches, or configs in general, is desired. Just define a `docker-compose.yaml` for each config and then simply spinning down and up stacks is all that is needed to switch between them.

> [!WARNING]
> I advise against running multiple instances of nHentai Archivist at the same time.

## Usage
### Download a Few Quickly

1. Run the program as is. Do not set `NHENTAI_TAGS` and make sure there is no file at `DOWNLOADME_FILEPATH`.
1. Enter the nHentai ID you want to download separated by spaces. This might not work using docker.

Example `./config/.env`:

```TOML
LIBRARY_PATH = "./hentai/"
```

### Download a Bit More From a File

1. Do not set `NHENTAI_TAGS`.
1. Set `DOWNLOADME_FILEPATH` and create a file there.
1. Enter the nHentai ID you want to download separated by linebreaks.

Example `./config/.env`:

```TOML
CSRFTOKEN = "your token here"
DONTDOWNLOADME_FILEPATH = "./config/dontdownloadme.txt"
DOWNLOADME_FILEPATH = "./config/downloadme.txt"
LIBRARY_PATH = "./hentai/"
USER_AGENT = "your user agent here"
```

### Ich mein's ernst: Keeping a Self-Hosted Library Up-to-Date

1. Set `NHENTAI_TAGS` to define the tag search that you want to use to keep your library up-to-date.
1. Set `DOWNLOADME_FILEPATH` but ensure there is no file actually there, otherwise it will be downloaded first.
1. Consider setting `LIBRARY_SPLIT` to a value other than 0 if you plan to keep more than 10.000 files in your `LIBRARY_PATH` directory.
1. Consider setting `SLEEP_INTERVAL` to wait a bit between searches. I recommend a value of at least 50.000.
1. Searching by tag results in seemingly random error 404 on some pages. Let it search and download multiple times to get everything reliably.

Example `./config/.env`:

```TOML
CF_CLEARANCE = ""
CSRFTOKEN = "your token here"
DONTDOWNLOADME_FILEPATH = "./config/dontdownloadme.txt"
DOWNLOADME_FILEPATH = "./config/downloadme.txt"
LIBRARY_PATH = "./hentai/"
LIBRARY_SPLIT = 10000
NHENTAI_TAGS = ['language:"english"']
SLEEP_INTERVAL = 50000
USER_AGENT = "your user agent here"
```

## Exporting Favourites

nHentai Archivist is not connected to your nHentai account in any way. Automatically generating a `downloadme.txt` from your list of favourites is beyond the scope of this tool. However, once you compiled your favourites as a list of ID separated by line breaks, nHentai Archivist can take over. Other users were quick to automate this process, I have linked a few of the provided solutions:

- https://www.reddit.com/r/DataHoarder/comments/1fg5yzy/comment/ln3m11g/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

- https://www.reddit.com/r/animepiracy/comments/1fg6crs/comment/ln4dwvv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

- https://github.com/phillychi3/nhentai-favorites

> [!CAUTION]
> I have not tested any of these scripts. I am linking them in good faith and rely on the feedback of the community.

## Known Issues

- ~~Searching by tags / downloading metadata often results in error 404 on seemingly random pages.\
This behaviour is consistent even when the URL is opened by a browser, so I assume the problem to be on nHentai's side. Just ignore the warnings and let nHentai Archivist search and download multiple times to get everything reliably, ideally with a `SLEEP_INTERVAL` of at least 50.000 so searches are guaranteed to be far enough apart. After a few runs, you will notice all but the newest hentai being skipped during the download phase. That's when you know you got everything. See [issue #3](https://github.com/9-FS/nhentai_archivist/issues/3).~~\
Seems to have been fixed upstream 2024-10-30.

- nHentai contains a lot of duplicates.\
    There is currently no way to filter them out. Setting `CLEANUP_TEMPORARY_FILES` has been added to improve interoperability  with third party deduplication software. See [issue #6](https://github.com/9-FS/nhentai_archivist/issues/6).

- The [nHentai search API](https://nhentai.net/info/) does not match whole words only; or at least I can't figure out how. For example, searching the artist "mana" will download "mana-ko" and "aoi manabu", everything with mana in it. This does not affect (normal) tags as far as I have experienced.\
    I recommend to always double check your `NHENTAI_TAGS` with a search on nhentai.net. See [issue #10](https://github.com/9-FS/nhentai_archivist/issues/10).

- Multiple tag searches can not be combined via logical OR.\
    The current workaround is to have multiple `docker-compose.yaml` each defining a different search by defining `NHENTAI_TAGS` in their environment section and then playing round robin with the container that is being used. See [issue #11](https://github.com/9-FS/nhentai_archivist/issues/11).

- I have upgraded from an older version and now nHentai Archivist complains about missing or invalid settings.\
    Either follow the error message's instructions and manually make the necessary changes or delete `./config/.env` and execute the program once to create a new default one.

- I have upgraded from an older version to version 3.8.0 or newer and now nHentai Archivist raises database errors (code 1).\
    This is due to me adding migrations to the database which enables me to change the database schema in the future without you having to do anything yourself. This is my very first project that utilises a proper database which is why I haven't utilised migrations from the start, I apologise for the hassle. To fix the error either add the migrations entry manually to the database, which is kinda complicated, or just delete the `./db/` directory to create a new database that works.

- 177013 "Metamorphosis" has been purged!\
    Apparently, only the gallery page has been purged and the images themselves are still available on the media servers; as long as you happen to remember the media ID from before the purge _wink wink_. By executing the following SQL commands, you can manually add the required metadata to the database and then download the images as usual. Huge thanks to a friendly redditor for this information.

    ```SQL
    INSERT
    INTO Hentai (id, cover_type, media_id, num_favorites, num_pages, page_types, scanlator, title_english, title_japanese, title_pretty, upload_date)
    VALUES (177013, 'j', media_id, 0, 225, 'jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj', NULL, '[ShindoLA] METAMORPHOSIS (Complete) [English]', NULL, 'METAMORPHOSIS', '2016-10-19T00:00:00Z');

    INSERT
    INTO Hentai_Tag (hentai_id, tag_id)
    VALUES
    (177013, 8010),
    (177013, 14283),
    (177013, 24201),
    (177013, 10314),
    (177013, 13720),
    (177013, 29859),
    (177013, 13989),
    (177013, 22942),
    (177013, 22945),
    (177013, 19018),
    (177013, 20035),
    (177013, 29224),
    (177013, 27384),
    (177013, 8739),
    (177013, 7256),
    (177013, 6343),
    (177013, 22079),
    (177013, 12695),
    (177013, 5820),
    (177013, 29182),
    (177013, 25050),
    (177013, 32996),
    (177013, 10542),
    (177013, 53449),
    (177013, 7288),
    (177013, 13722),
    (177013, 21112),
    (177013, 3981),
    (177013, 17249),
    (177013, 12227),
    (177013, 33173);
    ```
