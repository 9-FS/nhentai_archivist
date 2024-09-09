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


impl HentaiSearchResponse
{
    /// # Summary
    /// Write hentai search response to database. Either creates new entries or updates existing ones with same primary key.
    ///
    /// # Arguments
    /// - `db`: SQLite database
    ///
    /// # Returns
    /// - nothing or sqlx::Error
    pub async fn write_to_db(&self, db: &sqlx::sqlite::SqlitePool) -> Result<(), sqlx::Error>
    {
        let mut query: sqlx::query::Query<'_, _, _>; // query to update all tables
        let query_string: String; // sql query string


        let hentai_query_string: String = // query string for Hentai table
            "INSERT OR REPLACE INTO Hentai (id, cover_type, media_id, num_favorites, num_pages, page_types, scanlator, title_english, title_japanese, title_pretty, upload_date) VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);".to_owned();
        let tag_query_string: String = format! // query string for Tag table
        (
            "INSERT OR REPLACE INTO Tag (id, name, type, url) VALUES\n{};",
            self.tags.iter().map(|_| "(?, ?, ?, ?)").collect::<Vec<&str>>().join(",\n")
        );
        let hentai_tag_query_string: String = format! // query string for Hentai_Tag table
        (
            "DELETE FROM Hentai_Tag WHERE hentai_id = ?;\nINSERT INTO Hentai_Tag (hentai_id, tag_id) VALUES\n{};", // delete all Hentai_Tag entries with same hentai_id before in case hentai had some tags untagged
            self.tags.iter().map(|_| "(?, ?)").collect::<Vec<&str>>().join(",\n")
        );
        query_string = format!("PRAGMA foreign_keys = OFF;\nBEGIN TRANSACTION;\n{}\n{}\n{}\nCOMMIT;\nPRAGMA foreign_keys = ON;", hentai_query_string, tag_query_string, hentai_tag_query_string); // combine all tables into one transaction, foreign key validation is too slow for inserts at this scale

        query = sqlx::query(query_string.as_str());
        query = query // bind Hentai values to placeholders
            .bind(self.id)
            .bind(format!("{:?}", self.images.cover.t))
            .bind(self.media_id)
            .bind(self.num_favorites)
            .bind(self.num_pages)
            .bind(self.images.pages.iter().map(|page| format!("{:?}", page.t)).collect::<Vec<String>>().join("")) // collapse all page types into 1 string, otherwise have to create huge Hentai_Pages table or too many Hentai_{id}_Pages tables
            .bind(self.scanlator.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten()) // convert Some("") to None, otherwise forward unchanged
            .bind(self.title.english.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
            .bind(self.title.japanese.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
            .bind(self.title.pretty.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
            .bind(self.upload_date);

        for tag in self.tags.iter() // bind Tag values to placeholders
        {
            query = query
                .bind(tag.id)
                .bind(tag.name.clone())
                .bind(tag.r#type.clone())
                .bind(tag.url.clone());
        }

        query = query.bind(self.id); // bind hentai id to placeholder
        for tag in self.tags.iter() // bind Hentai_Tag values to placeholders
        {
            query = query
                .bind(self.id)
                .bind(tag.id);
        }

        query.execute(db).await?;
        return Ok(());
    }
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
    /// Write tag search response to database. Either creates new entries or updates existing ones with same primary key.
    ///
    /// # Arguments
    /// - `db`: SQLite database
    ///
    /// # Returns
    /// - nothing or sqlx::Error
    pub async fn write_to_db(&self, db: &sqlx::sqlite::SqlitePool) -> Result<(), sqlx::Error> // separate function to create 1 big transaction instead of 1 transaction per Hentai, tag search requires this performance
    {
        let mut query: sqlx::query::Query<'_, _, _>; // query to update all tables
        let query_string: String; // sql query string


        let hentai_query_string: String = format! // query string for Hentai table
        (
            "INSERT OR REPLACE INTO Hentai (id, cover_type, media_id, num_favorites, num_pages, page_types, scanlator, title_english, title_japanese, title_pretty, upload_date) VALUES\n{};",
            self.result.iter().map(|_| "(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").collect::<Vec<&str>>().join(",\n")
        );
        let tag_query_string: String = format!
        (
            "INSERT OR REPLACE INTO Tag (id, name, type, url) VALUES\n{};", // query string for Tag table
            self.result.iter().flat_map(|hentai| hentai.tags.iter()).map(|_| "(?, ?, ?, ?)").collect::<Vec<&str>>().join(",\n")
        );
        let mut hentai_tag_query_string: String = String::new(); // query string for Hentai_Tag table
        for hentai in self.result.iter()
        {
            hentai_tag_query_string += format!
            (
                "DELETE FROM Hentai_Tag WHERE hentai_id = ?;\nINSERT INTO Hentai_Tag (hentai_id, tag_id) VALUES\n{};", // delete all Hentai_Tag entries with same hentai_id before in case hentai had some tags untagged
                hentai.tags.iter().map(|_| "(?, ?)").collect::<Vec<&str>>().join(",\n")
            ).as_str();
        }
        query_string = format!("PRAGMA foreign_keys = OFF;\nBEGIN TRANSACTION;\n{}\n{}\n{}\nCOMMIT;\nPRAGMA foreign_keys = ON;", hentai_query_string, tag_query_string, hentai_tag_query_string); // combine all tables into one transaction, foreign key validation is too slow for inserts at this scale

        query = sqlx::query(query_string.as_str());

        for hentai in self.result.iter() // bind Hentai values to placeholders
        {
            query = query
                .bind(hentai.id)
                .bind(format!("{:?}", hentai.images.cover.t))
                .bind(hentai.media_id)
                .bind(hentai.num_favorites)
                .bind(hentai.num_pages)
                .bind(hentai.images.pages.iter().map(|page| format!("{:?}", page.t)).collect::<Vec<String>>().join("")) // collapse all page types into 1 string, otherwise have to create huge Hentai_Pages table or too many Hentai_{id}_Pages tables
                .bind(hentai.scanlator.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten()) // convert Some("") to None, otherwise forward unchanged
                .bind(hentai.title.english.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
                .bind(hentai.title.japanese.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
                .bind(hentai.title.pretty.clone().map(|s| if s.is_empty() {None} else {Some(s)}).flatten())
                .bind(hentai.upload_date);
        }

        for hentai in self.result.iter() // bind Tag values to placeholders
        {
            for tag in hentai.tags.iter()
            {
                query = query
                    .bind(tag.id)
                    .bind(tag.name.clone())
                    .bind(tag.r#type.clone())
                    .bind(tag.url.clone());
            }
        }

        for hentai in self.result.iter() // bind Hentai_Tag values to placeholders
        {
            query = query.bind(hentai.id); // bind hentai id to placeholder
            for tag in hentai.tags.iter()
            {
                query = query
                    .bind(hentai.id)
                    .bind(tag.id);
            }
        }

        query.execute(db).await?;
        return Ok(());
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