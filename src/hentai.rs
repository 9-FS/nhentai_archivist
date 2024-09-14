// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use crate::api_response::*;
use crate::comicinfoxml::*;
use crate::error::*;
use crate::search_api::*;
use std::io::Read;
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use unicode_segmentation::UnicodeSegmentation;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hentai
{
    pub id: u32, // nhentai.net hentai id
    pub cbz_filename: String, // filename of temporary and final cbz
    pub gallery_url: String, // nhentai.net gallery url
    pub images_filename: Vec<String>, // filenames of images, not filepath because needed at temporary image location and final zip location
    pub images_url: Vec<String>, // urls to images to download
    pub library_path: String, // path to local hentai library, relevant to generate filepaths of temporary images
    pub library_split: u32, // split library into subdirectories with maximum this number of hentai, 0 for no split
    pub num_pages: u16,
    pub scanlator: Option<String>,
    pub tags: Vec<Tag>, // tags from Tag table, must be broken up by type
    pub title_pretty: Option<String>,
    pub upload_date: chrono::DateTime<chrono::Utc>
}


impl Hentai
{
    /// # Summary
    /// Tries to build a Hentai with the metadata from the following sources with respective descending priority:
    /// 1. if entry exists in database: load from database
    /// 1. by searching on nhentai.net for a hentai with ID `id`
    ///
    /// # Arguments
    /// - `id`: hentai ID
    /// - `db`: database connection
    /// - `http_client`: reqwest http client
    /// - `nhentai_hentai_search_url`: nhentai.net hentai search API URL
    /// - `library_path`: path to local hentai library
    /// - `library_split`: split library into subdirectories with maximum this number of hentai, 0 for no split
    ///
    /// # Returns
    /// - created hentai or error
    pub async fn new(id: u32, db: &sqlx::sqlite::SqlitePool, http_client: &reqwest::Client, nhentai_hentai_search_url: &str, library_path: &str, library_split: u32) -> Result<Self, HentaiNewError>
    {
        const FILENAME_SIZE_MAX: u16 = 255; // maximum filename size [B]
        const TITLE_CHARACTERS_FORBIDDEN: &str = "\\/:*?\"<>|\t\n"; // forbidden characters in Windows file names
        let mut cbz_filename: String;
        let hentai_table_row: HentaiTableRow;
        let mut images_filename: Vec<String> = Vec::new();
        let mut images_url: Vec<String> = Vec::new();
        let tags: Vec<Tag>;


        if let Ok(Some(s)) = sqlx::query_as("SELECT id, media_id, num_pages, page_types, scanlator, title_english, title_pretty, upload_date FROM Hentai WHERE id = ?")
            .bind(id)
            .fetch_optional(db).await // load hentai metadata from database
        {
            hentai_table_row = s;
            log::info!("Loaded hentai metadata from database.");
        }
        else // if any step to load from database failed
        {
            log::info!("Hentai metadata could not be loaded from database. Downloading from nhentai.net API...");
            hentai_table_row = search_by_id(http_client, nhentai_hentai_search_url, id, db).await?; // load hentai metadata from api
            log::info!("Downloaded hentai metadata.");
        }

        tags = sqlx::query_as("SELECT Tag.* FROM Tag JOIN (SELECT tag_id FROM Hentai_Tag WHERE hentai_id = ?) AS tags_attached_to_hentai_desired ON Tag.id = tags_attached_to_hentai_desired.tag_id")
            .bind(id)
            .fetch_all(db).await?; // load tags from database
        log::info!("Loaded hentai tags from database.");


        for (i, page_type) in hentai_table_row.page_types.char_indices()
        {
            images_url.push(format!("https://i.nhentai.net/galleries/{}/{}.{}", hentai_table_row.media_id, i+1, ImageType::from_str(page_type.to_string().as_str()).expect("Invalid image type even though it was loaded from the database.")));
            images_filename.push(format!("{id}-{:05}.{}", i+1, ImageType::from_str(page_type.to_string().as_str()).expect("Invalid image type even though it was loaded from the database.")));
        }
        if hentai_table_row.page_types.len() != hentai_table_row.num_pages as usize // if number of pages does not match number of page types: inconsistency
        {
            return Err(HentaiNewError::HentaiLengthInconsistency {page_types: hentai_table_row.page_types.len() as u16, num_pages: hentai_table_row.num_pages});
        }

        cbz_filename = hentai_table_row.title_english.clone().unwrap_or_default();
        cbz_filename.retain(|c| !TITLE_CHARACTERS_FORBIDDEN.contains(c)); // remove forbidden characters
        if FILENAME_SIZE_MAX - 12 < cbz_filename.len() as u16 // if title size problematic
        {
            let mut byte_count: u16 = 0;
            cbz_filename = cbz_filename
                .graphemes(true) // iterate over graphemes
                .take_while (|&g| // only add grapheme if it wouldn't bust limit
                {
                    byte_count += g.len() as u16;
                    byte_count <= FILENAME_SIZE_MAX - 12
                }) // limit title to 243 B so filename does not exceed 255 B
                .collect();
        }
        cbz_filename = format!("{id} {cbz_filename}.cbz"); // prepend id, append extension

        Ok(Self
        {
            id,
            cbz_filename,
            gallery_url: format!("https://nhentai.net/g/{id}/"),
            images_filename,
            images_url,
            library_path: library_path.to_owned(),
            library_split,
            num_pages: hentai_table_row.num_pages,
            scanlator: hentai_table_row.scanlator,
            tags,
            title_pretty: hentai_table_row.title_pretty,
            upload_date: hentai_table_row.upload_date,
        })
    }


    /// # Summary
    /// Downloads all images of the hentai and combines them into a cbz file.
    ///
    /// # Arguments
    /// - `http_client`: reqwest http client
    /// - `db`: database connectionc
    ///
    /// # Returns
    /// - nothing or error
    pub async fn download(&self, http_client: &reqwest::Client) -> Result<(), HentaiDownloadError>
    {
        const WORKERS: usize = 5; // number of parallel workers
        let cbz_final_filepath: String; //filepath to final cbz in library
        let cbz_temp_filepath: String = format!("{}{}/{}", self.library_path, self.id, self.cbz_filename); //filepath to temporary cbz, cbz is created here and when finished moved to final location, roundabout way over temporary cbz filepath in case program gets stopped while creating cbz, so no half finished cbz remains in library
        let f = scaler::Formatter::new()
            .set_scaling(scaler::Scaling::None)
            .set_rounding(scaler::Rounding::Magnitude(0)); // formatter
        let mut image_download_success: bool = true; // if all images were downloaded successfully, redundant initialisation here because of stupid error message
        let mut handles: Vec<tokio::task::JoinHandle<Option<()>>>; // list of handles to download_image
        let worker_sem: std::sync::Arc<tokio::sync::Semaphore> = std::sync::Arc::new(tokio::sync::Semaphore::new(WORKERS)); // limit number of concurrent workers otherwise api enforces rate limit
        let mut zip_writer: zip::ZipWriter<std::fs::File>; // write to zip file


        if self.library_split == 0 // no library split
        {
            cbz_final_filepath = format!("{}{}", self.library_path, self.cbz_filename);
        }
        else // with library split
        {
            cbz_final_filepath = format!
            (
                "{}{}~{}/{}",
                self.library_path.to_owned(),
                self.id.div_euclid(self.library_split) * self.library_split,
                (self.id.div_euclid(self.library_split) + 1) * self.library_split - 1,
                self.cbz_filename,
            );
        }

        if let Ok(o) = tokio::fs::metadata(cbz_final_filepath.as_str()).await
        {
            if o.is_file() // if final cbz already exists
            {
                log::info!("Hentai already exists. Skipped download.");
                return Ok(()); // skip download
            }
            if o.is_dir() // if cbz filepath blocked by directory
            {
                return Err(HentaiDownloadError::BlockedByDirectory {directory_path: cbz_final_filepath.clone()}); // give up
            }
        }


        for _ in 0..5 // try to download hentai maximum 5 times
        {
            image_download_success = true; // assume success
            handles = Vec::new(); // reset handles

            for i in 0..self.images_url.len() // for each page
            {
                let f_clone: scaler::Formatter = f.clone();
                let http_client_clone: reqwest::Client = http_client.clone();
                let image_filepath: String = format!("{}{}/{}", self.library_path, self.id, self.images_filename.get(i).expect("Index out of bounds even though should have same size as images_url."));
                let image_url_clone: String = self.images_url.get(i).expect("Index out of bounds even though checked before that it fits.").clone();
                let num_pages_clone: u16 = self.num_pages;

                let permit: tokio::sync::OwnedSemaphorePermit = worker_sem.clone().acquire_owned().await.expect("Something closed semaphore even though it should never be closed."); // acquire semaphore
                handles.push(tokio::spawn(async move
                {
                    let result: Option<()>;
                    match Self::download_image(&http_client_clone, &image_url_clone, &image_filepath).await // download image
                    {
                        Ok(_) =>
                        {
                            log::debug!("Downloaded hentai image {} / {}.", f_clone.format((i+1) as f64), f_clone.format(num_pages_clone as f64));
                            result = Some(()); // success
                        }
                        Err(e) =>
                        {
                            log::warn!("{e}");
                            result = None; // failure
                        }
                    }
                    drop(permit); // release semaphore
                    result // return result into handle
                })); // search all pages in parallel
            }
            for handle in handles
            {
                if let None = handle.await.unwrap() {image_download_success = false;} // collect results, forward panics, if any image download failed: set flag and abandon creation of cbz later but continue downloading other images
            }
            if image_download_success {break;} // if all images were downloaded successfully: continue with cbz creation
        }
        if !image_download_success {return Err(HentaiDownloadError::Download {})}; // if after 5 attempts still not all images downloaded successfully: give up
        log::info!("Downloaded hentai images.");


        let zip_file: std::fs::File;
        #[cfg(target_family = "unix")]
        {
            tokio::fs::DirBuilder::new().recursive(true).mode(0o777).create(std::path::Path::new(format!("{}{}", self.library_path, self.id).as_str())).await?; // create all parent directories with permissions "drwxrwxrwx"
            zip_file = std::fs::OpenOptions::new().create(true).mode(0o666).write(true).open(cbz_temp_filepath.clone())?; // create temporary cbz file with permissions "rw-rw-rw-", overwrite if already exists
            if let Err(e) = zip_file.set_permissions(std::fs::Permissions::from_mode(0o666)) // set permissions
            {
                log::warn!("Setting permissions \"rw-rw-rw-\"for hentai {} failed with: {e}", self.id);
            }
        }
        #[cfg(not(target_family = "unix"))]
        {
            tokio::fs::DirBuilder::new().recursive(true).create(std::path::Path::new(format!("{}{}", self.library_path, self.id).as_str())).await?; // create all parent directories
            zip_file = std::fs::OpenOptions::new().create(true).write(true).open(cbz_temp_filepath.clone())?; // create temporary cbz file, overwrite if already exists
        }

        zip_writer = zip::ZipWriter::new(zip_file); // create cbz writer
        for (i, image_filename) in self.images_filename.iter().enumerate() // load images into cbz
        {
            let mut image: Vec<u8> = Vec::new();
            std::fs::File::open(format!("{}{}/{image_filename}", self.library_path, self.id))?.read_to_end(&mut image)?; // open image file, read image into memory
            zip_writer.start_file(image_filename, zip::write::SimpleFileOptions::default().unix_permissions(0o666))?; // create image file in cbz with permissions "rw-rw-rw-"
            zip_writer.write_all(&image)?; // write image into cbz
            log::debug!("Saved hentai image {} / {} in cbz.", f.format((i+1) as f64), f.format(self.num_pages));
        }
        #[cfg(target_family = "unix")]
        zip_writer.start_file("ComicInfo.xml", zip::write::SimpleFileOptions::default().unix_permissions(0o666))?; // create metadata file in cbz with permissions "rw-rw-rw-"
        #[cfg(not(target_family = "unix"))]
        zip_writer.start_file("ComicInfo.xml", zip::write::SimpleFileOptions::default())?; // create metadata file in cbz without permissions
        zip_writer.write_all(serde_xml_rs::to_string(&ComicInfoXml::from(self.clone()))?.as_bytes())?; // write metadata into cbz
        zip_writer.finish()?; // finish temporary cbz

        #[cfg(target_family = "unix")]
        {
            if let Some(parent) = std::path::Path::new(cbz_final_filepath.as_str()).parent() {tokio::fs::DirBuilder::new().recursive(true).mode(0o777).create(parent).await?;} // create all parent directories of with permissions "drwxrwxrwx"
        }
        #[cfg(not(target_family = "unix"))]
        {
            if let Some(parent) = std::path::Path::new(cbz_final_filepath.as_str()).parent() {tokio::fs::DirBuilder::new().recursive(true).create(parent).await?;} // create all parent directories
        }
        tokio::fs::rename(cbz_temp_filepath, cbz_final_filepath).await?; // move finished cbz to final location in library
        log::info!("Saved hentai cbz.");


        if let Err(e) = tokio::fs::remove_dir_all(format!("{}{}", self.library_path, self.id)).await // cleanup, delete image directory
        {
            log::warn!("Deleting \"{}/\" failed with: {e}", format!("{}{}", self.library_path, self.id));
        }

        Ok(())
    }


    /// # Summary
    /// Downloads an image from `image_url` and saves it to `image_filepath`.
    ///
    /// # Arguments
    /// - `http_client`: reqwest http client
    /// - `image_url`: url of the image to download
    /// - `image_filepath`: path to save the image to
    ///
    /// # Returns
    /// - nothing or error
    async fn download_image(http_client: &reqwest::Client, image_url: &str, image_filepath: &str) -> Result<(), HentaiDownloadImageError>
    {
        if let Ok(o) = tokio::fs::metadata(image_filepath).await
        {
            if o.is_file() {return Ok(());} // if image already exists: skip download
            if o.is_dir() {return Err(HentaiDownloadImageError::BlockedByDirectory {directory_path: image_filepath.to_owned()});} // if image filepath blocked by directory: give up
        }


        let r: reqwest::Response = http_client.get(image_url).send().await?; // tag search, page
        if r.status() != reqwest::StatusCode::OK {return Err(HentaiDownloadImageError::ReqwestStatus {url: r.url().to_string(), status: r.status()});} // if status is not ok: something went wrong


        let mut file: tokio::fs::File;
        #[cfg(target_family = "unix")]
        {
            if let Some(parent) = std::path::Path::new(image_filepath).parent() // create all parent directories with permissions "drwxrwxrwx"
            {
                if let Err(e) = tokio::fs::DirBuilder::new().recursive(true).mode(0o777).create(parent).await
                {
                    return Err(HentaiDownloadImageError::StdIo {filepath: image_filepath.to_owned(), source: e});
                }
            }
            match tokio::fs::OpenOptions::new().create_new(true).mode(0o666).write(true).open(image_filepath).await
            {
                Ok(o) => file = o,
                Err(e) => {return Err(HentaiDownloadImageError::StdIo {filepath: image_filepath.to_owned(), source: e});}
            }
        }
        #[cfg(not(target_family = "unix"))]
        {
            if let Some(parent) = std::path::Path::new(image_filepath).parent() // create all parent directories
            {
                if let Err(e) = tokio::fs::DirBuilder::new().recursive(true).create(parent).await
                {
                    return Err(HentaiDownloadImageError::StdIo {filepath: image_filepath.to_owned(), source: e});
                }
            }
            match tokio::fs::OpenOptions::new().create_new(true).write(true).open(image_filepath).await
            {
                Ok(o) => file = o,
                Err(e) => {return Err(HentaiDownloadImageError::StdIo {filepath: image_filepath.to_owned(), source: e});}
            }
        }

        if let Err(e) = file.write_all_buf(&mut r.bytes().await?).await // save image with permissions "rw-rw-rw-"
        {
            return Err(HentaiDownloadImageError::StdIo {filepath: image_filepath.to_owned(), source: e});
        }

        Ok(())
    }
}


#[derive(Clone, Debug, Eq, PartialEq, sqlx::FromRow)]
pub struct HentaiTableRow
{
    pub id: u32,
    pub media_id: u32,
    pub num_pages: u16,
    pub page_types: String,
    pub scanlator: Option<String>,
    pub title_english: Option<String>,
    pub title_pretty: Option<String>,
    pub upload_date: chrono::DateTime<chrono::Utc>,
}