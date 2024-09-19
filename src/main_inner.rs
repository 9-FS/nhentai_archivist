// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::config::*;
use crate::connect_to_db::*;
use crate::error::*;
use crate::get_hentai_id_list::*;
use crate::hentai::*;


pub async fn main_inner(config: Config) -> Result<(), Error>
{
    const HTTP_TIMEOUT: u64 = 30; // connection timeout
    const NHENTAI_HENTAI_SEARCH_URL: &str = "https://nhentai.net/api/gallery/"; // nhentai search by id api url
    const NHENTAI_TAG_SEARCH_URL: &str = "https://nhentai.net/api/galleries/search"; // nhentai search by tag api url
    let f0 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(0)); // formatter
    let fm2 = scaler::Formatter::new()
        .set_scaling(scaler::Scaling::None)
        .set_rounding(scaler::Rounding::Magnitude(-2)); // formatter
    let f4 = scaler::Formatter::new(); // formatter


    loop // keep running for server mode
    {
        {
            let db: sqlx::sqlite::SqlitePool; // database containing all metadata from nhentai.net api
            let hentai_id_list: Vec<u32>; // list of hentai id to download
            let http_client: reqwest::Client; // http client


            let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new(); // headers
            match reqwest::header::HeaderValue::from_str(&config.USER_AGENT)
            {
                Ok(o) => _ = headers.insert(reqwest::header::USER_AGENT, o),
                Err(e) => log::warn!("Adding user agent to HTTP client headers failed with: {e}\nUsing empty user agent instead."),
            }
            match reqwest::header::HeaderValue::from_str(format!("cf_clearance={}; csrftoken={}", config.CF_CLEARANCE, config.CSRFTOKEN).as_str())
            {
                Ok(o) => _ = headers.insert(reqwest::header::COOKIE, o),
                Err(e) => log::warn!("Adding cookies \"cf_clearance\" and \"csrftoken\" to HTTP client headers failed with: {e}\nUsing no cookies instead."),
            }
            match reqwest::Client::builder()  // create http client
                .connect_timeout(std::time::Duration::from_secs(HTTP_TIMEOUT))
                .cookie_store(true) // enable cookies
                .default_headers(headers)
                .read_timeout(std::time::Duration::from_secs(HTTP_TIMEOUT))
                .build()
            {
                Ok(o) => http_client = o,
                Err(e) => return Err(Error::ReqwestClientBuilder {source: e}),
            }
            let r: reqwest::Response = http_client.get(NHENTAI_TAG_SEARCH_URL).query(&[("query", "language:english"), ("page", "1")]).send().await?; // send test request
            if
                r.status() != reqwest::StatusCode::OK  // if status is not ok
                && r.status() != reqwest::StatusCode::NOT_FOUND // and except for not found and too many requests: something went wrong, abort
                && r.status() != reqwest::StatusCode::TOO_MANY_REQUESTS // not found included because of nhentai api's random 404 fuckywuckys
            {
                return Err(Error::ReqwestStatus {url: r.url().to_string(), status: r.status()});
            }

            db = connect_to_db(&config.DATABASE_URL).await?; // connect to database
            hentai_id_list = get_hentai_id_list
            (
                config.DOWNLOADME_FILEPATH.as_str(),
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


                match Hentai::new(*hentai_id, &db, &http_client, NHENTAI_HENTAI_SEARCH_URL, &config.LIBRARY_PATH, config.LIBRARY_SPLIT).await
                {
                    Ok(o) => hentai = o, // hentai created successfully
                    Err(e) => // hentai creation failed
                    {
                        log::error!("{e}");
                        continue; // skip download
                    }
                }

                if let Err(e) = hentai.download(&http_client, config.CLEANUP_TEMPORARY_FILES).await
                {
                    log::error!{"{e}"};
                }
            }
            log::info!("--------------------------------------------------");


            db.close().await; // close database connection
            log::info!("Disconnected from database at \"{}\".", config.DATABASE_URL);

            if config.NHENTAI_TAGS.is_none() {break;} // if tag not set: client mode, exit

            if let Err(e) = tokio::fs::remove_file(&config.DOWNLOADME_FILEPATH).await // server mode cleanup, delete downloadme
            {
                log::warn!("Deleting \"{}\" failed with: {e}", config.DOWNLOADME_FILEPATH);
            }
        } // free as much memory as possible

        log::info!("Sleeping for {}s...", f4.format(config.SLEEP_INTERVAL.unwrap_or_default() as f64));
        tokio::time::sleep(std::time::Duration::from_secs(config.SLEEP_INTERVAL.unwrap_or_default())).await; // if in server mode: sleep for interval until next check
        log::info!("--------------------------------------------------");
    }

    return Ok(());
}