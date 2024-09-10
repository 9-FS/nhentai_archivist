// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::config::*;
use crate::connect_to_db::*;
use crate::error::*;
use crate::get_hentai_id_list::*;
use crate::hentai::*;


pub async fn main_inner(config: Config) -> Result<()>
{
    const NHENTAI_HENTAI_SEARCH_URL: &str="https://nhentai.net/api/gallery/"; // nhentai search by id api url
    const NHENTAI_TAG_SEARCH_URL: &str="https://nhentai.net/api/galleries/search"; // nhentai search by tag api url
    let mut db: sqlx::sqlite::SqlitePool; // database containing all metadata from nhentai.net api
    let f0 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(0)); // formatter
    let fm2 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(-2)); // formatter
    let f4 = scaler::Formatter::new(); // formatter
    let mut hentai_id_list: Vec<u32>; // list of hentai id to download
    let http_client: reqwest::Client; // http client
    let timeout: std::time::Duration = std::time::Duration::from_secs(30); // connection timeout


    {
        let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new(); // headers
        headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(&config.USER_AGENT).unwrap_or_else
        (
            |e|
            {
                log::warn!("Adding user agent to HTTP client headers failed with: {e}\nUsing empty user agent instead.");
                reqwest::header::HeaderValue::from_str("").expect("Creating empty user agent failed.")
            }
        ));
        headers.insert(reqwest::header::COOKIE, reqwest::header::HeaderValue::from_str(format!("cf_clearance={}; csrftoken={}", config.CF_CLEARANCE, config.CSRFTOKEN).as_str()).unwrap_or_else
        (
            |e|
            {
                log::warn!("Adding cookies \"cf_clearance\" and \"csrftoken\" to HTTP client headers failed with: {e}\nUsing no cookies instead.");
                reqwest::header::HeaderValue::from_str("").expect("Creating empty cookies failed.")
            }
        ));
        http_client = reqwest::Client::builder()  // create http client
            .connect_timeout(timeout)
            .cookie_store(true) // enable cookies
            .default_headers(headers)
            .read_timeout(timeout)
            .build().expect("Creating HTTP client failed.");
        let r: reqwest::Response = http_client.get(NHENTAI_TAG_SEARCH_URL).query(&[("query", "language:english"), ("page", "1")]).send().await?; // send test request
        if r.status() != reqwest::StatusCode::OK // if status is not ok: something went wrong
        {
            return Err(Error::ReqwestStatus {url: r.url().to_string(), status: r.status()});
        }
    }


    loop // keep running for server mode
    {
        db = connect_to_db(&config.DATABASE_URL).await?; // connect to database
        hentai_id_list = get_hentai_id_list
        (
            std::path::Path::new(config.DOWNLOADME_FILEPATH.as_str()),
            &http_client,
            NHENTAI_TAG_SEARCH_URL,
            &config.NHENTAI_TAG,
            &db,
        ).await;


        for (i, hentai_id) in hentai_id_list.iter().enumerate()
        {
            log::info!("--------------------------------------------------");
            log::info!("{} / {} ({})", f0.format((i+1) as f64), f0.format(hentai_id_list.len() as f64), fm2.format((i+1) as f64 / hentai_id_list.len() as f64));
            let hentai: Hentai; // hentai to download


            if (i + 1).rem_euclid(1000) == 0 // reconnect to database every 1000 downloads to free up resources
            {
                db.close().await; // close database connection
                log::info!("Disconnected from database at \"{}\".", config.DATABASE_URL);
                db = connect_to_db(&config.DATABASE_URL).await?; // reconnect to database
            }

            match Hentai::new(*hentai_id, &db, &http_client, NHENTAI_HENTAI_SEARCH_URL, &config.LIBRARY_PATH, config.LIBRARY_SPLIT).await
            {
                Ok(o) => hentai = o, // hentai created successfully
                Err(e) => // hentai creation failed
                {
                    match e
                    {
                        Error::HentaiLengthInconsistency { page_types, num_pages } => log::error!("Hentai {hentai_id} has {} page types specified, but {} pages were expected.", f0.format(page_types), f0.format(num_pages)),
                        Error::Reqwest(e) => log::error!("Hentai {hentai_id} metadata could not be loaded from database and downloading from \"{}\" failed with: {e}", e.url().map_or_else(|| "<unknown>", |o| o.as_str())),
                        Error::ReqwestStatus {url, status} => log::error!("Hentai {hentai_id} metadata could not be loaded from database and downloading from \"{url}\" failed with status code {status}."),
                        Error::SerdeJson(e) => log::error!("Hentai {hentai_id} metadata could not be loaded from database and after downloading, deserialising API response failed with: {e}"),
                        Error::Sqlx(e) => log::error!("Loading hentai {hentai_id} tags from database failed with: {e}"),
                        _ => panic!("Unhandled error: {e}"),
                    }
                    continue; // skip download
                }
            }

            if let Err(e) = hentai.download(&http_client).await
            {
                match e
                {
                    Error::BlockedByDirectory {directory_path} => {log::error!("Downloading hentai {hentai_id} failed, because \"{directory_path}\" already is a directory.");} // directory blocked
                    Error::Download {} => log::error!("Downloading hentai {hentai_id} failed multiple times. Giving up..."), // download failed multiple times, more specific error messages already in download logged
                    Error::SerdeXml(e) => log::error!("Serialising hentai {hentai_id} metadata failed with: {e}"), // serde xml error
                    Error::StdIo(e) => log::error!("Saving hentai {hentai_id} failed with: {e}"), // std io error
                    Error::Zip(e) => log::error!("Saving hentai {hentai_id} failed with: {e}"), // zip error
                    _ => panic!("Unhandled error: {e}"),
                }
            }
        }
        log::info!("--------------------------------------------------");


        if config.NHENTAI_TAG.is_none() {break;} // if tag not set: client mode, exit

        if let Err(e) = tokio::fs::remove_file(&config.DOWNLOADME_FILEPATH).await // server mode cleanup, delete downloadme
        {
            log::error!("Deleting \"{}\" failed with: {e}", config.DOWNLOADME_FILEPATH);
        }
        db.close().await; // close database connection
        log::info!("Disconnected from database at \"{}\".", config.DATABASE_URL);

        log::info!("Sleeping for {}s...", f4.format(config.SLEEP_INTERVAL.unwrap_or_default() as f64));
        tokio::time::sleep(std::time::Duration::from_secs(config.SLEEP_INTERVAL.unwrap_or_default())).await; // if in server mode: sleep for interval until next check
        log::info!("--------------------------------------------------");
    }

    return Ok(());
}

// https://nhentai.net/api/galleries/search?query=language%3Aenglish