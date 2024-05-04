use super::books::Book;
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::{future::Future, io, path::PathBuf};
use thiserror::Error;

pub trait ChapterDispatcher {
    fn get_chapter(
        &self,
        book: &Book,
        chapter_num: usize,
    ) -> impl Future<Output = Result<Bible, ChapterError>> + Send;
}

pub struct IndonesianBible;

#[derive(Debug, Error)]
pub enum ChapterError {
    #[error("Error fetching chapter content: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Error I/O chapter content: {0}")]
    IOError(#[from] io::Error),

    #[error("Content deserialization error: {0}")]
    DeserializationError(#[from] serde_xml_rs::Error),
}

impl ChapterDispatcher for IndonesianBible {
    async fn get_chapter(&self, book: &Book, chapter_num: usize) -> Result<Bible, ChapterError> {
        let chapters_path = PathBuf::from("./chapters");
        if !chapters_path.exists() {
            std::fs::create_dir(&chapters_path)
                .inspect_err(|e| tracing::error!("can't create 'chapters' directory: {}", e))?;
        }
        let cached_path = chapters_path.join(format!("id_{}_{}", book.index(), chapter_num));
        if cached_path.exists() {
            let res = std::fs::read_to_string(cached_path)
                .inspect_err(|e| tracing::error!("can't read cached file: {}", e))?;
            Ok(serde_xml_rs::from_str::<Bible>(&res)
                .inspect_err(|e| tracing::error!("can't parse XML: {}", e))?)
        } else {
            let url = format!(
                "https://alkitab.sabda.org/api/chapter.php?book={}&chapter={}",
                book.index(),
                chapter_num
            );
            let res = ReqwestClient::new()
                .get(url)
                .send()
                .await
                .inspect_err(|e| tracing::error!("can't send reqwest: {}", e))?
                .text()
                .await
                .inspect_err(|e| tracing::error!("can't get text from response: {}", e))?;
            std::fs::write(cached_path, &res)
                .inspect_err(|e| tracing::error!("can't write cache file: {}", e))?;
            Ok(serde_xml_rs::from_str::<Bible>(&res)
                .inspect_err(|e| tracing::error!("can't parse xml: {}", e))?)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Bible {
    pub title: String,
    pub book: u32,
    pub bookname: String,
    pub chapter: u32,
    pub chapter_count: u32,
    pub verses: Verses,
}

#[derive(Debug, Deserialize)]
pub struct Verses {
    pub verse: Vec<Verse>,
}

#[derive(Debug, Deserialize)]
pub struct Verse {
    pub number: u32,
    pub title: Option<String>,
    pub text: String,
}
