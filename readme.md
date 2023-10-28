---
Topic: "nHentai to PDF"
Author: "구FS"
---
<link href="./doc_templates/md_style.css" rel="stylesheet"></link>
<body>

# <p style="text-align: center">nHentai to PDF</p>
<br>
<br>

- [1. General](#1-general)
- [2. How to Set Up](#2-how-to-set-up)
  - [2.1. Firefox](#21-firefox)
  - [2.2. Google Chrome](#22-google-chrome)
- [3. How to Use](#3-how-to-use)

## 1. General

This is the nHentai downloader I wrote to archive as much of the [english nHentai library](https://nhentai.net/language/english/popular) as I can. That's why from the beginning it has been designed with big data sizes in mind and, for example, uses multithreaded downloads to download more than 1 image at once. Still, I wanted to keep it as simple code-wise and as easy to use as I can; hope I succeeded with that.  
Big thanks go out to [h3nTa1m4st3r_xx69](https://github.com/sam-k0), who helped me using nhentai's completely undocumented API. Without him this project could not have been reactivated.  
I'm happy about anyone who finds my software useful and feedback is also always welcome. Happy downloading~

<div style="page-break-after: always;"></div>

## 2. How to Set Up
### 2.1. Firefox

1. Execute the program once. This will create a default `./config/cookies.json`.
1. Go to https://nhentai.net/. Clear the cloudflare prompt.
1. Open the developer console with F12.
1. Go to the tab "Storage". On the left side expand "Cookies". Click on "https://nhentai.net".
1. Copy the cookie values into the `cookies.json`.
1. Execute the program again. This will create a default `.config/headers.json`.
1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `headers.json`.
1. In `./config/settings.json` set `dest_path` to the directory you want to download to. By default, it will download to `./hentai/`.

### 2.2. Google Chrome

1. Execute the program once. This will create a default `./config/cookies.json`.
1. Go to https://nhentai.net/. Clear the cloudflare prompt.
1. Open the developer console with F12.
1. Go to the tab "Application". On the left side under "Storage", expand "Cookies". Click on "https://nhentai.net".
1. Copy the cookie values into the `cookies.json`.
1. Execute the program again. This will create a default `.config/headers.json`.
1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `headers.json`.
1. In `./config/settings.json` set `dest_path` to the directory you want to download to. By default, it will download to `./hentai/`.

> :information_source:  
> Setting cookies seems to be required daily nowadays.

<div style="page-break-after: always;"></div>

## 3. How to Use

Choose hentai by nHentai ID to download and convert to PDF. You can either load ID from a `./config/downloadme.txt` seperated by linebreaks or directly enter ID into the console separated by spaces.

</body>