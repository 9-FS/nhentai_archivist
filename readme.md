# nhentai_archivist
## Introduction

nHentai Archivist is a tool to download hentai from https://nhentai.net and convert them to CBZ files. From quickly downloading a few hentai specified in the console, downloading a few hundred hentai specified in a downloadme.txt, up to automatically keeping a massive self-hosted library up-to-date by automatically generating a downloadme.txt from a search by tag. (For that use-case it has been optimised to tag the CBZ files in a way that [Komga](https://komga.org/) in [oneshot mode](https://komga.org/docs/guides/oneshots) interprets everything correctly.)

nHentai Archivist is not connected to your nHentai account in any way. Automatically generating a `downloadme.txt` from your list of favourites is beyond the scope of this tool. However, once you compiled your favourites as a list of ID separated by line breaks, nHentai Archivist can take over.

Why CBZ? CBZ is a widespread standard and basically just a ZIP file containing the images and a metadata file. This enables NHentai Archivist to **keep the tags** which wouldn't be possible with PDF as far as I know.

Big thanks go out to [h3nTa1m4st3r_xx69](https://github.com/sam-k0), who helped me with using nhentai's completely undocumented API. Without him this project could not have been reactivated.
I'm happy about anyone who finds my software useful and feedback is also always welcome. Happy downloading~

## Installation

1. Execute the program once to create a default `./config/.env`. This means that in the directory of the executable, there should now be a directory called "config" containing a file called ".env". You might need to enable seeing hidden files in the file explorer.
1. Confirm the database directory at `DATABASE_URL` exists, which is `./db/` by default. It is possible that it is not created automatically because the URL could point to a remote directory. The database file will and should be created automatically.
1. If you have problems with nhentai's bot protection (error 403), set `CF_CLEARANCE`, `CSRFTOKEN`, and `USER_AGENT`. Though I recommend setting the latter 2 in any case.

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
> If nhentai has "under attack" mode enabled, clearing the Cloudflare prompt and updating `CF_CLEARANCE` seem to be required daily.

Further settings:

- `NHENTAI_TAG`

    Setting this will trigger "server mode". If no file at `DOWNLOADME_FILEPATH` is found, it will generate one by searching for the tag specified. After all hentai on the downloadme have been downloaded, it will wait for `SLEEP_INTERVAL` seconds and restart the search. This is useful to keep a self-hosted library up-to-date with the latest releases from the specified tag.

    Examples:

    - "NHENTAI_TAG = language:english": all english hentai
    - "NHENTAI_TAG = tag:big-breasts": all hentai with the tag "big breasts"
    - "NHENTAI_TAG = parody:kono-subarashii-sekai-ni-syukufuku-o": all hentai from the anime "Kono Subarashii Sekai ni Syukufuku o"
    - "NHENTAI_TAG = artist:shindol": all hentai by Shindol

    More information can be found [here](https://nhentai.net/info/).

- `LIBRARY_PATH`

    This is the directory temporary images and finished CBZ files are download to. By default, it will download to `./hentai/`.

- `LIBRARY_SPLIT`

    Setting this to a value other than 0 splits the library at `LIBRARY_PATH` into sub-directories with a maximum number of `LIBRARY_SPLIT` hentai allowed per sub-directory. It is recommended if the number of hentai in 1 directory starts to affect file explorer performance. This _should_ not affect you if you plan to keep less than 10.000 files in your `LIBRARY_PATH` directory, otherwise the recommended setting is "LIBRARY_SPLIT = 10000".

## Usage
### Download a Few Quickly

1. Run the program as is. Do not set `NHENTAI_TAG` and make sure there is no file at `DOWNLOADME_FILEPATH`.
1. Enter the nhentai ID you want to download separated by spaces.

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

1. Do not set `NHENTAI_TAG`.
1. Create a file at `DOWNLOADME_FILEPATH` and enter the nhentai ID you want to download separated by linebreaks.

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

1. Set `NHENTAI_TAG` to the tag you want to keep up-to-date. For a very comprehensive library, set it to "NHENTAI_TAG = language:english".
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
NHENTAI_TAG = "language:english"
SLEEP_INTERVAL = 50000
USER_AGENT = your user agent here
```