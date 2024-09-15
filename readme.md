# nhentai_archivist
## Introduction

nHentai Archivist is a tool to download hentai from https://nhentai.net and convert them to CBZ files. From quickly downloading a few hentai specified in the console, downloading a few hundred hentai specified in a downloadme.txt, up to automatically keeping a massive self-hosted library up-to-date by automatically generating a downloadme.txt from a search by tag. (For that use-case it has been optimised to tag the CBZ files in a way that [Komga](https://komga.org/) in [oneshot mode](https://komga.org/docs/guides/oneshots) interprets everything correctly.)

Why CBZ? CBZ is a widespread standard and basically just a ZIP file containing the images and a metadata file. This enables NHentai Archivist to **keep the tags** which wouldn't be possible with PDF as far as I know.

Big thanks go out to [h3nTa1m4st3r_xx69](https://github.com/sam-k0), who helped me with using nHentai's completely undocumented API. Without him this project could not have been reactivated.
I'm happy about anyone who finds my software useful and feedback is also always welcome. Happy downloading~

## Quick Start

1. Execute the program once to create a default `./config/.env`.
1. Execute the program again and enter the nHentai ID you want to download separated by spaces.

## Installation

1. Execute the program once to create a default `./config/.env`.\
    This means that in the directory of the executable, there should now be a directory called "config" containing a file called ".env". You might need to enable seeing hidden files in the file explorer.
1. I recommend setting the `CSRFTOKEN` cookie and `USER_AGENT`. If you start having problems with nHentai's bot protection (error 403), setting these is mandatory. If a `CF_CLEARANCE` cookie is available, it should be set as well.

    ### Mozilla Firefox

    1. Go to https://nhentai.net/. Clear the Cloudflare prompt.
    1. Open the developer console with F12.
    1. Go to the tab "Storage". On the left side expand "Cookies". Click on "https://nhentai.net".
    1. Copy the cookie values into `./config/.env`.
    1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `./config/.env`.

    ### Google Chrome

    1. Go to https://nhentai.net/. Clear the cloudflare prompt.
    1. Open the developer console with F12.
    1. Go to the tab "Application". On the left side under "Storage", expand "Cookies". Click on "https://nhentai.net".
    1. Copy the cookie values into `./config/.env`.
    1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `./config/.env`.

> [!NOTE]
> If nHentai has "under attack" mode enabled, clearing the Cloudflare prompt and updating `CF_CLEARANCE` seem to be required daily.

Further settings:

- `DATABASE_URL`

    This is the path to the SQLite database file. If you changed `DATABASE_URL`, confirm the database directory already exists. It is possible that it is not created automatically because the URL could point to a remote directory. The database file will and should be created automatically.


- `NHENTAI_TAGS`

    Setting this will trigger "server mode". If no file at `DOWNLOADME_FILEPATH` is found, it will generate one by searching for the tags specified. After all hentai on the downloadme have been downloaded, it will wait for `SLEEP_INTERVAL` seconds and restart the search. This is useful to keep a self-hosted library up-to-date with the latest releases from the specified tag search. Multiple tags and tag exclusions can be specified and are connected via logical AND. This means results must fullfill all criteria specified.

    Examples:

    ```TOML
    NHENTAI_TAGS = ['language:"english"'] # all english hentai
    NHENTAI_TAGS = ['tag:"big breasts"'] # all hentai with the tag "big breasts"
    NHENTAI_TAGS = ['parody:"kono subarashii sekai ni syukufuku o"'] # all hentai from the anime "Kono Subarashii Sekai ni Syukufuku o"
    NHENTAI_TAGS = ['artist:"shindol"'] #all hentai by Shindol
    NHENTAI_TAGS = ['character:"frieren"'] # all hentai with character "Frieren"
    NHENTAI_TAGS = ['tag:"ffm threesome"', 'tag:"sister"', '-tag:"full censorship"', '-tag:"mind control"'] # all hentai with the tags "ffm threesome" and "sister" but without the tags "full censorship" and "mind control"
    ```

    Pay attention to copy the format exactly as shown in the examples. That includes the usage of single quotation marks outside and double quotation marks inside. If the format is not being copied exactly, at least searching by tags that contain a space leads to erroneous API responses. More information can be found [here](https://nhentai.net/info/).

- `LIBRARY_PATH`

    This is the directory temporary images and finished CBZ files are download to. By default, it will download to `./hentai/`.

- `LIBRARY_SPLIT`

    Setting this to a value other than 0 splits the library at `LIBRARY_PATH` into sub-directories with a maximum number of `LIBRARY_SPLIT` hentai allowed per sub-directory. It is recommended if the number of hentai in 1 directory starts to affect file explorer performance. This _should_ not affect you if you plan to keep less than 10.000 files in your `LIBRARY_PATH` directory, otherwise the recommended setting is `LIBRARY_SPLIT = 10000`.

## Usage
### Download a Few Quickly

1. Run the program as is. Do not set `NHENTAI_TAGS` and make sure there is no file at `DOWNLOADME_FILEPATH`.
1. Enter the nHentai ID you want to download separated by spaces. This might not work using docker.

Example `./config/.env`:

```TOML
CF_CLEARANCE = ""
CSRFTOKEN = your token here
DATABASE_URL = "./db/db.sqlite"
DOWNLOADME_FILEPATH = "./config/downloadme.txt"
LIBRARY_PATH = "./hentai/"
LIBRARY_SPLIT = 0
USER_AGENT = your user agent here
```

### Download a Bit More From a File

1. Do not set `NHENTAI_TAGS`.
1. Create a file at `DOWNLOADME_FILEPATH` and enter the nHentai ID you want to download separated by linebreaks.

Example `./config/.env`:

```TOML
CF_CLEARANCE = ""
CSRFTOKEN = your token here
DATABASE_URL = "./db/db.sqlite"
DOWNLOADME_FILEPATH = "./config/downloadme.txt"
LIBRARY_PATH = "./hentai/"
LIBRARY_SPLIT = 0
USER_AGENT = your user agent here
```

### Ich mein's ernst: Keeping a Self-Hosted Library Up-to-Date

1. Set `NHENTAI_TAGS` to define the tag search that you want to use to keep your library up-to-date.
1. Make sure there is no file at `DOWNLOADME_FILEPATH` otherwise it will be downloaded first.
1. Consider setting `LIBRARY_SPLIT` to a value other than 0 if you plan to keep more than 10.000 files in your `LIBRARY_PATH` directory.
1. Consider setting `SLEEP_INTERVAL` to wait a bit between searches. I recommend a value of at least 50.000.
1. Searching by tag results in seemingly random error 404 on some pages. Let it search and download multiple times to get everything reliably.

Example `./config/.env`:

```TOML
CF_CLEARANCE = ""
CSRFTOKEN = your token here
DATABASE_URL = "./db/db.sqlite"
DOWNLOADME_FILEPATH = "./config/downloadme.txt"
LIBRARY_PATH = "./hentai/"
LIBRARY_SPLIT = 10000
NHENTAI_TAGS = ['language:"english"']
SLEEP_INTERVAL = 50000
USER_AGENT = your user agent here
```

## Exporting Favourites

nHentai Archivist is not connected to your nHentai account in any way. Automatically generating a `downloadme.txt` from your list of favourites is beyond the scope of this tool. However, once you compiled your favourites as a list of ID separated by line breaks, nHentai Archivist can take over. Other users were quick to automate this process, I have linked a few of the provided solutions:

- https://www.reddit.com/r/DataHoarder/comments/1fg5yzy/comment/ln3m11g/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

- https://www.reddit.com/r/animepiracy/comments/1fg6crs/comment/ln4dwvv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

- https://github.com/phillychi3/nhentai-favorites

> [!CAUTION]
> I have not tested any of these scripts. I am linking them in good faith and rely on the feedback of the community.

## Known Issues

- Searching by tags / downloading metadata often results in error 404 on seemingly random pages. This behaviour is consistent even when the URL is opened by a browser, so I assume the problem to be on nHentai's side. Just ignore the warnings and let nHentai Archivist search and download multiple times to get everything reliably, ideally with a `SLEEP_INTERVAL` of at least 50.000 so searches are guaranteed to be far enough apart. After a few runs, you will notice all but the newest hentai being skipped during the download phase. That's when you know you got everything. See [issue #3](https://github.com/9-FS/nhentai_archivist/issues/3).

- nHentai contains a lot of duplicates. There is currently no way to filter them out. See [issue #6](https://github.com/9-FS/nhentai_archivist/issues/6).

- The [nHentai search API](https://nhentai.net/info/) does not match whole words only; or at least I can't figure out how. For example, searching the artist "mana" will download "mana-ko" and "aoi manabu", everything with mana in it. This does not affect (normal) tags as far as I have experienced. I recommend to always double check your `NHENTAI_TAGS` with a search on nhentai.net. See [issue #10](https://github.com/9-FS/nhentai_archivist/issues/10).