// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
mod api_response;
mod comicinfoxml;
mod config;
use config::*;
mod connect_to_db;
mod error;
use error::*;
mod get_hentai_id_list;
mod hentai;
mod main_inner;
use main_inner::*;
mod search_api;


fn main() -> std::process::ExitCode
{
    const DEBUG: bool = false; // debug mode?
    let mut crate_logging_level: std::collections::HashMap<String, log::Level> = std::collections::HashMap::new(); // logging level for individual crates
    let config: Config; // config, settings
    let tokio_rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().expect("Creating tokio runtime failed."); // async runtime


    crate_logging_level.insert("hyper_util".to_owned(), log::Level::Info); // shut up
    crate_logging_level.insert("serde_xml_rs".to_owned(), log::Level::Error); // shut up
    crate_logging_level.insert("sqlx::query".to_owned(), log::Level::Error); // shut up
    if DEBUG == true // setup logging
    {
        setup_logging::setup_logging(log::Level::Debug, Some(crate_logging_level), "./log/%Y-%m-%dT%H_%M.log");
    }
    else
    {
        setup_logging::setup_logging(log::Level::Info, Some(crate_logging_level), "./log/%Y-%m-%d.log");
    }

    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| // override panic behaviour
    {
        log::error!("{}", panic_info); // log panic source and reason
        log::error!("{}", std::backtrace::Backtrace::capture()); // log backtrace
    }));

    match load_config::load_config
    (
        vec!
        [
            load_config::Source::Env,
            load_config::Source::File(load_config::SourceFile::Toml("./config/.env".to_string())),
        ],
        Some(load_config::SourceFile::Toml("./config/.env".to_string()))
    )
    {
        Ok(o) => {config = o;} // loaded config successfully
        Err(_) => {return std::process::ExitCode::FAILURE;} // loading config failed
    }


    match std::panic::catch_unwind(|| tokio_rt.block_on(main_inner(config.clone()))) // execute main_inner, catch panic
    {
        Ok(result) => // no panic
        {
            match result
            {
                Ok(()) => {return std::process::ExitCode::SUCCESS;} // program executed successfully
                Err(e) => // program failed in a controlled manner
                {
                    match e // log error
                    {
                        Error::Reqwest(e) => log::error!("Test connecting to \"{}\" failed with: {e}", e.url().map_or_else(|| "<unknown>", |o| o.as_str())),
                        Error::ReqwestStatus { url, status } =>
                        {
                            if status == reqwest::StatusCode::FORBIDDEN
                            {
                                log::error!("Test connecting to \"{url}\" failed with status code {status}. Check if cookies \"cf_clearance\" and \"csrftoken\" and user agent are set and current.");
                            }
                            else
                            {
                                log::error!("Test connecting to \"{url}\" failed with status code {status}.");
                            }
                        }
                        Error::Sqlx(e) => log::error!("Connecting to database at \"{}\" failed with: {e}\nIf you're creating a new database, ensure all parent directories already exist.", config.DATABASE_URL),
                        _ => panic!("Unhandled error: {e}"),
                    }
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
        Err(_) => {return std::process::ExitCode::FAILURE;} // program crashed with panic, dis not good
    };
}