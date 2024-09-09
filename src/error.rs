// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.


#[derive(Debug, thiserror::Error)]
pub enum Error
{
    #[error("{directory_path}")]
    BlockedByDirectory {directory_path: String},

    #[error("")]
    Download {},

    #[error("")]
    HentaiLengthInconsistency {page_types: u16, num_pages: u16},

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("{status}")]
    ReqwestStatus {url: String, status: reqwest::StatusCode},

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeXml(#[from] serde_xml_rs::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}


pub type Result<T> = std::result::Result<T, Error>; // strict error handling, only takes pre defined Error type