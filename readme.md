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
- [2. How to Setup (with Google Chrome)](#2-how-to-setup-with-google-chrome)
- [3. How to Use](#3-how-to-use)

## 1. General

This is the nHentai downloader I wrote to archive as much of the [english nHentai library](https://nhentai.net/language/english/popular) as I can. That's why from the beginning it has been designed with big data sizes in mind and, for example, uses multithreaded downloads to download more than 1 image at once. Still, I wanted to keep it as simple code-wise and as easy to use as I can; hope I succeeded with that. I'm happy about anyone who finds my software useful and feedback is also always welcome. Happy downloading~

## 2. How to Setup (with Google Chrome)

1. Execute the program once. This will create a default `cookies.json`.
1. With your normal browser, go to [nhentai.net](https://nhentai.net/). Clear the cloudflare prompt.
1. Open the developer console with F12.
1. Go to the tab "Application". On the left side under "Storage", expand "Cookies". Click on "https://nhentai.net".
1. Copy the cookie values into the `cookies.json`.
1. Execute the program again. This will create a default `headers.json`.
1. Go to https://www.whatismybrowser.com/detect/what-is-my-user-agent/ and copy your user agent into `headers.json`.


## 3. How to Use

Choose hentai by nHentai ID to download and convert to PDF. You can either load ID from a `./downloadme.txt` seperated by linebreaks or directly enter ID into the console separated by spaces.

</body>