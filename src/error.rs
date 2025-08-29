// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error("{reason}")]
    SettingInvalid {reason: String},

    #[error("Connecting to database failed with: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Test connecting to \"{}\" failed with: {}", .0.url().map_or_else(|| "<unknown>", |o| o.as_str()), .0)]
    Wreq(#[from] wreq::Error),

    #[error("Creating HTTP client failed with: {source}")]
    WreqClientBuilder {source: wreq::Error},

    #[error("Test connecting to \"{url}\" failed with status code {status}.")]
    WreqStatus {url: String, status: wreq::StatusCode},
}


#[derive(Debug, thiserror::Error)]
pub enum HentaiNewError
{
    #[error
    (
        "Hentai has {} page types specified, but {} pages were expected.",
        scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(*page_types),
        scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(*num_pages)
    )]
    HentaiLengthInconsistency {page_types: u16, num_pages: u16},

    #[error(transparent)]
    SearchById(#[from] SearchByIdError),

    #[error("Loading hentai tags from database failed with: {0}")]
    Sqlx(#[from] sqlx::Error),
}


#[derive(Debug, thiserror::Error)]
pub enum HentaiDownloadError
{
    #[error("Saving hentai failed, because \"{directory_path}\" already is a directory.")]
    BlockedByDirectory {directory_path: String}, // directory blocked

    #[error("Downloading hentai failed multiple times. Giving up...")]
    Download(), // download failed multiple times, more specific error messages already in download logged

    #[error("Serialising hentai metadata failed with: {0}")]
    SerdeXml(#[from] serde_xml_rs::Error), // serde xml error

    #[error("Saving hentai failed with: {0}")]
    StdIo(#[from] std::io::Error), // std io error

    #[error("Saving hentai failed with: {0}")]
    Zip(#[from] zip::result::ZipError), // zip error
}


#[derive(Debug, thiserror::Error)]
pub enum HentaiDownloadImageError
{
    #[error("Saving hentai image failed, because \"{directory_path}\" already is a directory.")]
    BlockedByDirectory {directory_path: String}, // directory blocked

    #[error("Downloading hentai image from \"{}\" failed with: {}", .0.url().map_or_else(|| "<unknown>", |o| o.as_str()), .0)]
    Wreq(#[from] wreq::Error),

    #[error("Downloading hentai image from \"{url}\" failed with status code {status}.")]
    WreqStatus {url: String, status: wreq::StatusCode},

    #[error("Saving hentai image at \"{filepath}\" failed with: {source}")]
    StdIo {filepath: String, source: std::io::Error},
}


#[derive(Debug, thiserror::Error)]
pub enum RemoveOnlyEmptyDirError
{
    #[error("Removing directory \"{path}\" failed with: {source}")]
    StdIo {path: String, source: std::io::Error},
}


#[derive(Debug, thiserror::Error)]
pub enum SearchByIdError
{
    #[error("Hentai metadata could not be loaded from database and downloading from \"{}\" failed with: {}", .0.url().map_or_else(|| "<unknown>", |o| o.as_str()), .0)]
    Wreq(#[from] wreq::Error),

    #[error("Hentai metadata could not be loaded from database and downloading from \"{url}\" failed with status code {status}.")]
    WreqStatus {url: String, status: wreq::StatusCode},

    #[error("Hentai metadata could not be loaded from database and after downloading, deserialising API response failed with: {0}")]
    SerdeJson(#[from] serde_json::Error),
}


#[derive(Debug, thiserror::Error)]
pub enum SearchByTagOnPageError
{
    #[error
    (
        "Downloading hentai metadata page {} / {} from \"{}\" failed with: {source}",
        scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(*page_no),
        num_pages.map_or("<unknown>".to_owned(), |o| scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(o)),
        source.url().map_or_else(|| "<unknown>", |o| o.as_str())
    )]
    Wreq {page_no: u32, num_pages: Option<u32>, source: wreq::Error},

    #[error
    (
        "Downloading hentai metadata page {} / {} from \"{url}\" failed with status code {status}.",
        scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(*page_no),
        num_pages.map_or("<unknown>".to_owned(), |o| scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(o)),
    )]
    WreqStatus {page_no: u32, num_pages: Option<u32>, url: String, status: wreq::StatusCode},

    #[error
    (
        "Saving hentai metadata page {} / {} in database failed with: {source}",
        scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(*page_no),
        num_pages.map_or("<unknown>".to_owned(), |o| scaler::Formatter::new().set_scaling(scaler::Scaling::None).set_rounding(scaler::Rounding::Magnitude(0)).format(o)),
    )]
    SerdeJson {page_no: u32, num_pages: Option<u32>, source: serde_json::Error},
}