// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::search_api::*;
use tokio::io::AsyncWriteExt;


/// # Summary
/// Tries to return hentai ID list to download from the following sources with respective descending priority:
/// 1. if it exists: load from `downloadme_filepath`
/// 1. if `nhentai_tags` set: by searching on nhentai.net for all hentai ID with tags from `nhentai_tags`
/// 1. manual user input, separated by spaces
///
/// # Arguments
/// - `downloadme_filepath`: path to file containing hentai ID list
/// - `dontdownloadme_filepath`: path to file containing hentai ID to not download, blacklist
/// - `http_client`: reqwest http client
/// - `nhentai_tag_search_url`: nhentai.net tag search API URL
/// - `nhentai_tags`: tags to search for
/// - `db`: database connection
///
/// # Returns
/// - list of hentai ID to download
pub async fn get_hentai_id_list(downloadme_filepath: &str, dontdownloadme_filepath: &Option<String>, http_client: &reqwest::Client, nhentai_tag_search_url: &str, nhentai_tags: Option<Vec<String>>, db: &sqlx::sqlite::SqlitePool) -> Vec<u32>
{
    let mut hentai_id_list: Vec<u32> = Vec::new(); // list of hentai id to download


    if std::path::Path::new(downloadme_filepath).is_file() // only try loading if downloadme_filepath actually exists, so only non trivial errors are logged with log::error!
    {
        match tokio::fs::read_to_string(downloadme_filepath).await // try to load downloadme
        {
            Ok(content) =>
            {
                hentai_id_list = content.lines().filter_map(|line| line.parse::<u32>().ok()).collect(); // String -> Vec<u32>, discard unparseable lines
                log::info!("Loaded hentai ID list from \"{downloadme_filepath}\".");
            },
            Err(e) => log::warn!("Loading hentai ID list from \"{downloadme_filepath}\" failed with: {e}"),
        };
    }
    else
    {
        log::info!("No hentai ID list found at \"{downloadme_filepath}\".");
    }
    if let Some(dontdownloadme_filepath) = dontdownloadme_filepath
    {
        hentai_id_list = remove_blacklisted_hentai_id(hentai_id_list, dontdownloadme_filepath).await; // remove blacklisted hentai id
    }
    if !hentai_id_list.is_empty() // if hentai_id_list is not empty: work is done
    {
        log::debug!("{hentai_id_list:?}");
        return hentai_id_list;
    }

    if nhentai_tags.is_some() // if nhentai_tags are set: search nhentai.net for hentai ID with tag
    {
        log::info!("\"NHENTAI_TAGS\" are set.");
        match search_by_tag
        (
            http_client,
            nhentai_tag_search_url,
            &nhentai_tags.expect("nhentai_tags lifting crashed even though previous line ensured Option is Some."),
            db,
        ).await
        {
            Ok(o) => hentai_id_list = o,
            Err(e) => log::error!("{e}"),
        }
    }
    else // if nhentai_tags are not set: request manual user input
    {
        log::info!("\"NHENTAI_TAGS\" are not set.");
    }
    if let Some(dontdownloadme_filepath) = dontdownloadme_filepath
    {
        hentai_id_list = remove_blacklisted_hentai_id(hentai_id_list, dontdownloadme_filepath).await; // remove blacklisted hentai id
    }
    if !hentai_id_list.is_empty() // if hentai_id_list is not empty: save tag search in downloadme.txt, work is done
    {
        #[cfg(target_family = "unix")]
        match tokio::fs::OpenOptions::new().create_new(true).mode(0o666).write(true).open(downloadme_filepath).await
        {
            Ok(mut file) =>
            {
                match file.write_all(hentai_id_list.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("\n").as_bytes()).await
                {
                    Ok(_) => log::info!("Saved hentai ID list from tag search at \"{downloadme_filepath}\"."),
                    Err(e) => log::warn!("Writing hentai ID list to \"{downloadme_filepath}\" failed with: {e}"),
                }
            },
            Err(e) => log::warn!("Saving hentai ID list at \"{downloadme_filepath}\" failed with: {e}"),
        }
        #[cfg(not(target_family = "unix"))]
        match tokio::fs::OpenOptions::new().create_new(true).write(true).open(downloadme_filepath).await
        {
            Ok(mut file) =>
            {
                match file.write_all(hentai_id_list.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("\n").as_bytes()).await
                {
                    Ok(_) => log::info!("Saved hentai ID list from tag search at \"{downloadme_filepath}\"."),
                    Err(e) => log::warn!("Writing hentai ID list to \"{downloadme_filepath}\" failed with: {e}"),
                }
            },
            Err(e) => log::warn!("Saving hentai ID list at \"{downloadme_filepath}\" failed with: {e}"),
        }
        log::debug!("{hentai_id_list:?}");
        return hentai_id_list;
    }

    loop // if everything else fails: request manual user input
    {
        log::info!("Enter the holy numbers: ");
        let mut input: String = String::new();
        _ = std::io::stdin().read_line(&mut input);
        log::debug!("{input}");
        hentai_id_list = input.split_whitespace()
            .filter_map(|line|
            {
                match line.parse::<u32>()
                {
                    Ok(o) => Some(o),
                    Err(e) =>
                    {
                        log::warn!("Parsing entry \"{line}\" to u32 failed with: {e}. Discarding...");
                        None
                    }
                }
            })
            .collect(); // String -> Vec<u32>, discard unparseable lines with warning

        if let Some(dontdownloadme_filepath) = dontdownloadme_filepath
        {
            hentai_id_list = remove_blacklisted_hentai_id(hentai_id_list, dontdownloadme_filepath).await; // remove blacklisted hentai id
        }
        if !hentai_id_list.is_empty() {break;} // if hentai_id_list is not empty: work is done
    }
    log::debug!("{hentai_id_list:?}");
    return hentai_id_list;
}


/// # Summary
/// Removes blacklisted hentai ID from list of hentai ID to download.
///
/// # Arguments
/// - `hentai_id_list`: list of hentai ID to download
/// - `dontdownloadme_filepath`: path to file containing hentai ID to not download, blacklist
///
/// # Returns
/// - list of hentai ID to download with blacklisted hentai ID removed
async fn remove_blacklisted_hentai_id(mut hentai_id_list: Vec<u32>, dontdownloadme_filepath: &str) -> Vec<u32>
{
    let mut blacklist: Vec<u32> = Vec::new(); // list of hentai id to not download


    if !std::path::Path::new(dontdownloadme_filepath).is_file() {return hentai_id_list;} // only try loading if dontdownloadme_filepath actually exists, so only non trivial errors are logged with log::error!

    match tokio::fs::read_to_string(dontdownloadme_filepath).await // try to load dontdownloadme
    {
        Ok(content) =>
        {
            blacklist = content.lines().filter_map(|line| line.parse::<u32>().ok()).collect(); // String -> Vec<u32>, discard unparseable lines
            log::debug!("Loaded hentai ID blacklist from \"{dontdownloadme_filepath}\".");
        },
        Err(e) => log::warn!("Loading hentai ID blacklist from \"{dontdownloadme_filepath}\" failed with: {e}"),
    };


    hentai_id_list.retain(|id| !blacklist.contains(id)); // remove blacklisted hentai id

    return hentai_id_list;
}