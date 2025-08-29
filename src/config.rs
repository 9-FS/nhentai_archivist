// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Collection of settings making up the configuration of the application.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Config
{
    pub CIRCUMVENT_LOAD_BALANCER: Option<bool>, // circumvent nhentai.net's load balancer at i.nhentai.net and directly use random media server, only use if load balancer is broken
    pub CLEANUP_TEMPORARY_FILES: Option<bool>, // clean up temporary files after downloading? some prefer off for deduplication or compatibility with other tools
    pub DEBUG: Option<bool>, // debug mode?
    pub DONTDOWNLOADME_FILEPATH: Option<String>, // path to file containing hentai ID to not download, blacklist
    pub DOWNLOAD_WORKERS: Option<usize>, // number of download workers for parallel image downloads
    pub DOWNLOADME_FILEPATH: Option<String>, // path to file containing hentai ID to download
    pub FILENAME_TITLE_TYPE: Option<TitleType>, // which title to use for filenames: English,Japanese,Pretty
    pub LIBRARY_PATH: String, // path to download hentai to
    pub LIBRARY_SPLIT: Option<u32>, // split library into subdirectories of maximum this many hentai, None or 0 to disable
    pub NHENTAI_TAGS: Option<Vec<String>>, // keep creating downloadme.txt from these tags and keep downloading (server mode), normal tags are in format "tag:{tag}" for example "tag:ffm-threesome"; if None: don't generate downloadme.txt, download hentai once (client mode)
    pub SLEEP_INTERVAL: Option<u64>, // sleep interval in seconds between checking for new hentai to download (server mode)
    pub USER_AGENT: Option<String>, // bypass bot protection
}

impl Default for Config
{
    fn default() -> Self
    {
        Self
        {
            CIRCUMVENT_LOAD_BALANCER: None, // no entry in default config, defaults to false
            CLEANUP_TEMPORARY_FILES: None,
            DEBUG: None, // no entry in default config, defaults to false
            DONTDOWNLOADME_FILEPATH: Some("./config/dontdownloadme.txt".to_owned()),
            DOWNLOAD_WORKERS: None, // no entry in default config, defaults to 5
            DOWNLOADME_FILEPATH: Some("./config/downloadme.txt".to_owned()),
            FILENAME_TITLE_TYPE: None, // no entry in default config, defaults to English
            LIBRARY_PATH: "./hentai/".to_owned(),
            LIBRARY_SPLIT: None, // no entry in default config, defaults to 0
            NHENTAI_TAGS: None,
            SLEEP_INTERVAL: Some(50000),
            USER_AGENT: Some("".to_owned()),
        }
    }
}


/// # Summary
/// Title type to use for hentai filenames.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TitleType
{
    English,
    Japanese,
    Pretty,
}

impl Default for TitleType
{
    fn default() -> Self {Self::English}
}