// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::config::*;
use crate::connect_to_db::*;
use crate::error::*;
use crate::get_hentai_id_list::*;
use crate::hentai::*;

pub async fn main_inner(config: Config) -> Result<(), Error> {
    const DB_FILEPATH: &str = "./db/db.sqlite"; // database filepath
    const HTTP_TIMEOUT: u64 = 10; // connection timeout
    const NHENTAI_HENTAI_SEARCH_URL: &str = "https://nhentai.net/api/gallery/"; // nhentai search by id api url
    const NHENTAI_TAG_SEARCH_URL: &str = "https://nhentai.net/api/galleries/search"; // nhentai search by tag api url
    let f0 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(0)); // formatter
    let fm2 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(-2)); // formatter
    let f4 = scaler::Formatter::new(); // formatter

    let mut browser: wreq_util::Emulation;

    'program: loop
    // keep running for server mode
    {
        'iteration: // particular iteration of gathering id to download and downloading, client mode does only does 1 iteration, server mode unlimited
            {
                let db: sqlx::sqlite::SqlitePool; // database containing all metadata from nhentai.net api
                let hentai_id_list: Vec<u32>; // list of hentai id to download
                let http_client: wreq::Client; // http client
                let cookie: &String;

                match &config.COOKIE
                {
                    Some(val) => cookie = val,
                    None =>
                        {
                            log::error!("No cookie set in config");
                            panic!()
                        }
                }

                let mut headers: wreq::header::HeaderMap = wreq::header::HeaderMap::new(); // headers
                match wreq::header::HeaderValue::from_str(config.USER_AGENT.as_deref().unwrap_or_default())
                {
                    Ok(o) => _ = headers.insert(wreq::header::USER_AGENT, o),
                    Err(e) =>
                        {
                            let ua = wreq::header::HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:141.0) Gecko/20100101 Firefox/141.0");
                            _ = headers.insert(wreq::header::USER_AGENT, ua.unwrap());
                            log::warn!("Adding user agent to HTTP client headers failed with: {e}\nUsing default user agent instead.");
                        }
                }
                match headers
                    .get(wreq::header::USER_AGENT)
                    .unwrap_or(&wreq::header::HeaderValue::from_str("DEFAULT").unwrap())
                    .to_str()
                    .unwrap_or_default()
                {
                    x if x.contains("firefox") => browser = wreq_util::Emulation::Firefox139,
                    x if x.contains("chrome") => browser = wreq_util::Emulation::Chrome137,
                    _ => browser = wreq_util::Emulation::Firefox139,
                }
                match wreq::header::HeaderValue::from_str(&cookie)
                {
                    Ok(o) => _ = headers.insert(wreq::header::COOKIE, o),
                    Err(e) =>
                        {
                            log::error!("Adding cookies to HTTP client headers failed with: {e}\nTerminating program.");
                            panic!();
                        }
                }
                match wreq::Client::builder()  // create http client
                    .emulation(browser)
                    .connect_timeout(std::time::Duration::from_secs(HTTP_TIMEOUT))
                    .cookie_store(true) // enable cookies
                    .default_headers(headers)
                    .read_timeout(std::time::Duration::from_secs(HTTP_TIMEOUT))
                    .build()
                {
                    Ok(o) => http_client = o,
                    Err(e) =>
                        {
                            if config.NHENTAI_TAGS.is_none() { return Err(Error::WreqClientBuilder { source: e }); } // if client mode: abort completely with error
                            log::error!("{}", Error::WreqClientBuilder {source: e});
                            break 'iteration; // if server mode: only abort iteration, go straight to sleeping
                        }
                }
                let r: wreq::Response;
                match http_client.get(NHENTAI_TAG_SEARCH_URL).query(&[("query", "language:english"), ("page", "1")]).send().await // send test request
                {
                    Ok(o) => r = o,
                    Err(e) =>
                        {
                            if config.NHENTAI_TAGS.is_none() { return Err(e.into()); } // if client mode: abort completely with error
                            log::error!("{e}");
                            break 'iteration; // if server mode: only abort iteration, go straight to sleeping
                        }
                }
                log::debug!("{}", r.status());
                if
                r.status() != wreq::StatusCode::OK  // if status is not ok
                    && r.status() != wreq::StatusCode::NOT_FOUND // and except for not found and too many requests: something went wrong, abort
                    && r.status() != wreq::StatusCode::TOO_MANY_REQUESTS // not found included because of nhentai api's random 404 fuckywuckys
                {
                    if config.NHENTAI_TAGS.is_none() { return Err(Error::WreqStatus { url: r.url().to_string(), status: r.status() }); } // if client mode: abort completely with error
                    log::error!("{}", Error::WreqStatus {url: r.url().to_string(), status: r.status()});
                    break 'iteration; // if server mode: only abort iteration, go straight to sleeping
                }

                match connect_to_db(DB_FILEPATH).await // connect to database
                {
                    Ok(o) => db = o,
                    Err(e) =>
                        {
                            if config.NHENTAI_TAGS.is_none() { return Err(e.into()); } // if client mode: abort completely with error
                            log::error!("{e}");
                            break 'iteration; // if server mode: only abort iteration, go straight to sleeping
                        }
                }
                hentai_id_list = get_hentai_id_list
                    (
                        &config.DOWNLOADME_FILEPATH,
                        &config.DONTDOWNLOADME_FILEPATH,
                        &http_client,
                        NHENTAI_TAG_SEARCH_URL,
                        config.NHENTAI_TAGS.clone(),
                        &db,
                    ).await;


                for (i, hentai_id) in hentai_id_list.iter().enumerate()
                {
                    log::info!("--------------------------------------------------");
                    log::info!("{} / {} ({}) | hentai {hentai_id}", f0.format((i+1) as f64), f0.format(hentai_id_list.len() as f64), fm2.format((i+1) as f64 / hentai_id_list.len() as f64));
                    let hentai: Hentai; // hentai to download


                    match Hentai::new // create hentai
                        (
                            *hentai_id,
                            &db,
                            &http_client,
                            NHENTAI_HENTAI_SEARCH_URL,
                            &config.FILENAME_TITLE_TYPE.clone().unwrap_or_default(), // if not set: default to english title
                            &config.LIBRARY_PATH,
                            config.LIBRARY_SPLIT.unwrap_or_default(), // use u32 with 0 to disable library split and not Option<u32> with None, because that would make Some(0) an invalid state
                        ).await
                    {
                        Ok(o) => hentai = o, // hentai created successfully
                        Err(e) => // hentai creation failed
                            {
                                log::error!("{e}");
                                continue; // skip download
                            }
                    }

                    if config.DOWNLOAD_WORKERS.unwrap_or(5) == 0 // if download workers is set to 0: nothing will download, abort
                    {
                        return Err(Error::SettingInvalid { reason: format!("Setting `DOWNLOAD_WORKERS` must have value greater than 0 or else nothing gets done.") });
                    }
                    if let Err(e) = hentai.download(&http_client, config.DOWNLOAD_WORKERS.unwrap_or(5), config.CIRCUMVENT_LOAD_BALANCER.unwrap_or(false), config.CLEANUP_TEMPORARY_FILES.unwrap_or(true)).await
                    {
                        log::error! {"{e}"};
                    }
                }
                log::info!("--------------------------------------------------");


                db.close().await; // close database connection
                log::info!("Disconnected from database at \"{}\".", DB_FILEPATH);

                if config.NHENTAI_TAGS.is_none() { break 'program; } // if tag not set: client mode, exit

                if let Some(s) = &config.DOWNLOADME_FILEPATH
                {
                    if let Err(e) = tokio::fs::remove_file(s).await // server mode cleanup, delete downloadme
                    {
                        log::warn!("Deleting \"{}\" failed with: {e}", s);
                    }
                }
            } // free as much memory as possible

        log::info!(
            "Sleeping for {}s...",
            f4.format(config.SLEEP_INTERVAL.unwrap_or_default() as f64)
        );
        tokio::time::sleep(std::time::Duration::from_secs(
            config.SLEEP_INTERVAL.unwrap_or_default(),
        ))
            .await; // if in server mode: sleep for interval until next check
        log::info!("--------------------------------------------------");
    }

    return Ok(());
}
