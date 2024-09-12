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
    pub NHENTAI_TAG: Option<String>, // keep creating downloadme.txt from this tag and keep downloading (server mode), normal tags are in format "tag:{tag}" for example "tag:ffm-threesome"; if None: don't generate downloadme.txt, download hentai once (client mode)
    pub SLEEP_INTERVAL: Option<u64>, // sleep interval in seconds between checking for new hentai to download (server mode)
    pub USER_AGENT: String, // bypass bot protection
}

impl Default for Config
{
    fn default() -> Self
    {
        Config
        {
            CF_CLEARANCE: "".to_string(),
            CSRFTOKEN: "".to_string(),
            DATABASE_URL: "sqlite://./db/db.sqlite".to_owned(),
            DEBUG: None, // no entry in default config, defaults to false
            DOWNLOADME_FILEPATH: "./config/downloadme.txt".to_owned(),
            LIBRARY_PATH: "./hentai/".to_string(),
            LIBRARY_SPLIT: 0,
            NHENTAI_TAG: None,
            SLEEP_INTERVAL: None,
            USER_AGENT: "".to_string(),
        }
    }
}