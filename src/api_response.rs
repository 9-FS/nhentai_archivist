// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use std::str::FromStr;


/// # Summary
/// Hentai search response from "nhentai.net/api/gallery/{id}".
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HentaiSearchResponse
{
    #[serde(deserialize_with = "try_str_to_u32")]
    pub id: u32,
    pub images: Images,
    #[serde(deserialize_with = "try_str_to_u32")]
    pub media_id: u32,
    pub num_favorites: u32,
    pub num_pages: u16,
    pub scanlator: Option<String>,
    pub tags: Vec<Tag>,
    pub title: Title,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub upload_date: chrono::DateTime<chrono::Utc>,
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Image
{
    pub h: u32,
    pub t: ImageType,
    pub w: u32,
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Images
{
    pub cover: Image,
    pub pages: Vec<Image>,
    pub thumbnail: Image,
}


#[derive(Clone, Eq, PartialEq)]
pub enum ImageType
{
    Gif,
    Jpg,
    Png,
    Webp,
}

impl<'de> serde::Deserialize<'de> for ImageType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> // str -> ImageType
    where
        D: serde::Deserializer<'de>,
    {
        let s_de: String = String::deserialize(deserializer)?;
        match Self::from_str(s_de.as_str())
        {
            Ok(o) => return Ok(o),
            _ => return Err(serde::de::Error::custom(format!("Invalid image type: \"{s_de}\""))),
        };
    }
}

impl serde::Serialize for ImageType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> // ImageType -> str
    where
        S: serde::Serializer,
    {
        let s: String = format!("{:?}", self);
        return serializer.serialize_str(s.as_str());
    }
}

impl std::fmt::Debug for ImageType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result // ImageType -> str
    {
        return write!(f, "{}",
            match self
            {
                Self::Gif => "g", // only short form in program context (database)
                Self::Jpg => "j",
                Self::Png => "p",
                Self::Webp => "w",
            }
        );
    }
}

impl std::fmt::Display for ImageType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result // ImageType -> str
    {
        return write!(f, "{}",
            match self
            {
                Self::Gif => "gif", // long form for output
                Self::Jpg => "jpg",
                Self::Png => "png",
                Self::Webp => "webp",
            }
        );
    }
}

impl std::str::FromStr for ImageType
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> // str -> ImageType
    {
        let image_type: ImageType = match s.to_lowercase().trim()
        {
            "g" | "gif" => Self::Gif,
            "j" | "jpg" => Self::Jpg,
            "p" | "png" => Self::Png,
            "w" | "webp" => Self::Webp,
            _ => return Err(format!("Invalid image type: \"{s}\"")),
        };
        return Ok(image_type);
    }
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Tag
{
    pub id: u32,
    pub name: String,
    pub r#type: String, // type is a reserved keyword, r#type resolves to type
    pub url: String,
}


/// # Summary
/// Tag search response from "nhentai.net/api/galleries/search?query={tag}".
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TagSearchResponse
{
    pub num_pages: u32,
    pub per_page: u16,
    pub result: Vec<HentaiSearchResponse>,
}

impl TagSearchResponse
{
    /// # Summary
    /// Write search reponse to database. A tag search response yields multiple hentai, a hentai search response 1. Either creates new entries or updates existing ones with same primary key.
    ///
    /// # Arguments
    /// - `db`: SQLite database
    ///
    /// # Returns
    /// - number of rows affected or sqlx::Error
    pub async fn write_to_db(search_response: Vec<HentaiSearchResponse>, db: &sqlx::sqlite::SqlitePool) -> Result<u64, sqlx::Error> // separate function to create 1 big transaction instead of 1 transaction per Hentai, tag search requires this performance
    {
        const HENTAI_QUERY_STRING: &str = "INSERT OR REPLACE INTO Hentai (id, cover_type, media_id, num_favorites, num_pages, page_types, scanlator, title_english, title_japanese, title_pretty, upload_date) "; // query string for Hentai table
        const HENTAI_TAG_QUERY1_STRING: &str = "DELETE FROM Hentai_Tag WHERE hentai_id IN "; // cleanup query string, delete all Hentai_Tag entries with same hentai_id before in case hentai had some tags untagged
        const HENTAI_TAG_QUERY2_STRING: &str = "INSERT INTO Hentai_Tag (hentai_id, tag_id) "; // query string for Hentai_Tag table
        const TAG_QUERY_STRING: &str = "INSERT OR REPLACE INTO Tag (id, name, type, url) "; // query string for Tag table
        let mut db_tx: sqlx::Transaction<'_, sqlx::Sqlite>; // transaction for all queries
        let mut rows_affected: u64 = 0; // number of rows affected by query


        db_tx = db.begin_with("PRAGMA foreign_keys = OFF; BEGIN TRANSACTION;").await?; // start transaction, disable foreign key checks for performance

        let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(HENTAI_QUERY_STRING); // query for Hentai table
        query.push_values
        (
            search_response.iter().filter(|hentai| !hentai.images.pages.is_empty()), // filter out all hentai without pages
            |mut builder, hentai|
            {
                builder
                    .push_bind(hentai.id)
                    .push_bind(format!("{:?}", hentai.images.cover.t))
                    .push_bind(hentai.media_id)
                    .push_bind(hentai.num_favorites)
                    .push_bind(hentai.num_pages)
                    .push_bind(hentai.images.pages.iter().map(|page| format!("{:?}", page.t)).collect::<Vec<String>>().join("")) // collapse all page types into 1 string, otherwise have to create huge Hentai_Pages table or too many Hentai_{id}_Pages tables
                    .push_bind(hentai.scanlator.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)})) // convert Some("") to None, otherwise forward unchanged
                    .push_bind(hentai.title.english.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)}))
                    .push_bind(hentai.title.japanese.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)}))
                    .push_bind(hentai.title.pretty.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)}))
                    .push_bind(hentai.upload_date);
            }
        );
        rows_affected += query
            .build()
            .persistent(false) // don't cache query
            .execute(&mut *db_tx).await? // execute query
            .rows_affected(); // get number of rows affected

        let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(TAG_QUERY_STRING); // query for Tag table
        query.push_values
        (
            search_response.iter().flat_map(|hentai| hentai.tags.iter()), // flatten all tags into 1 iterator
            |mut builder, tag|
            {
                builder
                    .push_bind(tag.id)
                    .push_bind(&tag.name)
                    .push_bind(&tag.r#type)
                    .push_bind(&tag.url);
            }
        );
        rows_affected += query
            .build()
            .persistent(false) // don't cache query
            .execute(&mut *db_tx).await? // execute query
            .rows_affected(); // get number of rows affected

        let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(HENTAI_TAG_QUERY1_STRING); // cleanup query for Hentai_Tag table
        query.push("(");
        for (i, hentai) in search_response.iter().enumerate()
        {
            query.push_bind(hentai.id);
            if i < search_response.len() - 1 // if not last element
            {
                query.push(", "); // append comma
            }
        }
        query.push(");\n");
        query.push(HENTAI_TAG_QUERY2_STRING); // query for Hentai_Tag table
        query.push_values
        (
            search_response.iter()
                .filter(|hentai| !hentai.images.pages.is_empty()) // no Hentai_Tag entries for hentai without pages
                .flat_map(|hentai| hentai.tags.iter().map(|tag| (hentai.id, tag.id))), // flatten all tags into 1 iterator, contains tuples of (hentai_id, tag_id)
            |mut builder, (hentai_id, tag_id)|
            {
                builder
                    .push_bind(hentai_id)
                    .push_bind(tag_id);
            }
        );
        rows_affected += query
            .build()
            .persistent(false) // don't cache query
            .execute(&mut *db_tx).await? // execute query
            .rows_affected(); // get number of rows affected


        db_tx.commit().await?; // commit transaction
        return Ok(rows_affected);
    }
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Title
{
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: Option<String>,
}


/// # Summary
/// Tries to parse a string or number into a u32.
///
/// # Arguments
/// - `deserializer`: serde deserializer
///
/// # Returns
/// - u32 or serde::Error
fn try_str_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    let number: u32;

    match value
    {
        serde_json::Value::Number(n) => number = n.as_u64().ok_or_else(|| serde::de::Error::custom(format!("Converting number {n} to u64 failed. Subsequent converion to u32 has been aborted.")))? as u32,
        serde_json::Value::String(s) => number = s.parse::<u32>().map_err(|e| serde::de::Error::custom(format!("Parsing string {s} to u32 failed with: {e}")))?,
        _ => return Err(serde::de::Error::custom(format!("Value \"{value}\" is neither a number nor a string.")))?,
    };

    return Ok(number);
}