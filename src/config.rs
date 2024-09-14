// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


/// # Summary
/// Collection of settings making up the configuration of the application.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[allow(non_snake_case)]
pub struct Config
{
    pub CF_CLEARANCE: String, // bypass bot protection
    pub CSRFTOKEN: String, // bypass bot protection
    pub DATABASE_URL: String, // url to database file
    pub DEBUG: Option<bool>, // debug mode?
    pub DOWNLOADME_FILEPATH: String, // path to file containing hentai ID to download
    pub LIBRARY_PATH: String, // path to download hentai to
    pub LIBRARY_SPLIT: u32, // split library into subdirectories of maximum this many hentai, 0 to disable
    pub NHENTAI_TAGS: Option<Vec<String>>, // keep creating downloadme.txt from these tags and keep downloading (server mode), normal tags are in format "tag:{tag}" for example "tag:ffm-threesome"; if None: don't generate downloadme.txt, download hentai once (client mode)
    pub SLEEP_INTERVAL: Option<u64>, // sleep interval in seconds between checking for new hentai to download (server mode)
    pub USER_AGENT: String, // bypass bot protection
}

impl Default for Config
{
    fn default() -> Self
    {
        Self
        {
            CF_CLEARANCE: "".to_owned(),
            CSRFTOKEN: "".to_owned(),
            DATABASE_URL: "./db/db.sqlite".to_owned(),
            DEBUG: None, // no entry in default config, defaults to false
            DOWNLOADME_FILEPATH: "./config/downloadme.txt".to_owned(),
            LIBRARY_PATH: "./hentai/".to_owned(),
            LIBRARY_SPLIT: 0,
            NHENTAI_TAGS: None,
            SLEEP_INTERVAL: Some(50000),
            USER_AGENT: "".to_owned(),
        }
    }
}