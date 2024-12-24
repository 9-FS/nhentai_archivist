CREATE TABLE Hentai
(
    id INTEGER NOT NULL,
    cover_type TEXT NOT NULL,
    media_id INTEGER NOT NULL,
    num_favorites INTEGER NOT NULL,
    num_pages INTEGER NOT NULL,
    page_types TEXT NOT NULL,
    scanlator TEXT,
    title_english TEXT,
    title_japanese TEXT,
    title_pretty TEXT,
    upload_date TEXT NOT NULL,
    PRIMARY KEY(id)
);
CREATE TABLE Tag
(
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    url TEXT NOT NULL,
    PRIMARY KEY(id)
);
CREATE TABLE Hentai_Tag
(
    hentai_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY(hentai_id, tag_id),
    FOREIGN KEY(hentai_id) REFERENCES Hentai(id),
    FOREIGN KEY(tag_id) REFERENCES Tag(id)
);