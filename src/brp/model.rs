use super::books::Book;
use crate::utils::today_naive_date;
use chrono::{Datelike, NaiveDate};
use lazy_static::lazy_static;
use sqlx::SqlitePool;

lazy_static! {
    static ref DEFULT_READINGS: Vec<Vec<Book>> = vec![
        vec![Book::Matthew, Book::Mark, Book::Luke, Book::John],
        vec![
            Book::Genesis,
            Book::Exodus,
            Book::Leviticus,
            Book::Numbers,
            Book::Deuteronomy,
        ],
        vec![
            Book::Romans,
            Book::FirstCorinthians,
            Book::SecondCorinthians,
            Book::Galatians,
            Book::Ephesians,
            Book::Philippians,
            Book::Colossians,
            Book::Hebrews,
        ],
        vec![
            Book::FirstThessalonians,
            Book::SecondThessalonians,
            Book::FirstTimothy,
            Book::SecondTimothy,
            Book::Titus,
            Book::Philemon,
            Book::James,
            Book::FirstPeter,
            Book::SecondPeter,
            Book::FirstJohn,
            Book::SecondJohn,
            Book::ThirdJohn,
            Book::Jude,
            Book::Revelation,
        ],
        vec![Book::Job, Book::Ecclesiastes, Book::SongOfSolomon],
        vec![Book::Psalms],
        vec![Book::Proverbs],
        vec![
            Book::Joshua,
            Book::Judges,
            Book::Ruth,
            Book::FirstSamuel,
            Book::SecondSamuel,
            Book::FirstKings,
            Book::SecondKings,
            Book::FirstChronicles,
            Book::SecondChronicles,
            Book::Ezra,
            Book::Nehemiah,
            Book::Esther,
        ],
        vec![
            Book::Isaiah,
            Book::Jeremiah,
            Book::Lamentations,
            Book::Ezekiel,
            Book::Daniel,
            Book::Hosea,
            Book::Joel,
            Book::Amos,
            Book::Obadiah,
            Book::Jonah,
            Book::Micah,
            Book::Nahum,
            Book::Habakkuk,
            Book::Zephaniah,
            Book::Haggai,
            Book::Zechariah,
            Book::Malachi,
        ],
        vec![Book::Acts],
    ];
}

#[derive(Debug)]
pub struct UserReadings {
    user_id: i64,
    pub readings: Vec<Vec<Book>>,
}

impl UserReadings {
    pub fn new(user_id: i64, readings: Vec<Vec<Book>>) -> Self {
        Self { user_id, readings }
    }

    pub fn new_with_default_readings(user_id: i64) -> Self {
        Self {
            user_id,
            readings: DEFULT_READINGS.clone(),
        }
    }

    /// Delete all readings for `self.id` from table `readings`
    /// and replace it with new `self.readings`.
    pub async fn replace_current(&self, pool: &SqlitePool) {
        let res = sqlx::query!("DELETE FROM readings WHERE user_id = ?", self.user_id)
            .execute(pool)
            .await
            .unwrap();
        println!("Affected rows: {}", res.rows_affected());
        for (i, reading) in self.readings.iter().enumerate() {
            let reading: Vec<_> = reading.into_iter().map(|s| s.to_string()).collect();
            let text = reading.join("|");

            let idx = i as i64;
            let res = sqlx::query!(
                "INSERT INTO readings (user_id, reading, idx) VALUES (?1, ?2, ?3)",
                self.user_id,
                text,
                idx
            )
            .execute(pool)
            .await
            .unwrap();
            println!("Inserted reading: {}", res.rows_affected());
        }
    }

    pub async fn from_user(pool: &SqlitePool, user_id: i64) -> Self {
        let records = sqlx::query!(
            "SELECT * FROM readings WHERE user_id = ? ORDER BY idx ASC",
            user_id
        )
        .fetch_all(pool)
        .await
        .unwrap();

        let mut readings = Vec::with_capacity(records.capacity());
        for rec in records {
            let reading: Vec<_> = rec
                .reading
                .split("|")
                .map(|s| s.parse::<Book>().unwrap())
                .collect();
            readings.push(reading);
        }
        Self { user_id, readings }
    }
}

#[derive(Debug)]
pub struct UserDates {
    pub start_date: NaiveDate,
    pub offset: i64,
}

impl UserDates {
    pub async fn set(pool: &SqlitePool, user_id: i64, start_date: NaiveDate, offset: i64) {
        sqlx::query!(
            "INSERT OR REPLACE INTO dates (user_id, start_date, offset) VALUES (?1, ?2, ?3)",
            user_id,
            start_date,
            offset
        )
        .execute(pool)
        .await
        .unwrap();
    }

    pub async fn set_default(pool: &SqlitePool, user_id: i64) -> Self {
        let reading_date = today_naive_date();
        let start_date =
            NaiveDate::from_ymd_opt(reading_date.year(), 1, 1).expect("valid year, month, day");
        UserDates::set(pool, user_id, start_date, 0).await;
        Self {
            start_date,
            offset: 0,
        }
    }

    pub async fn from_user_or_set_default(pool: &SqlitePool, user_id: i64) -> Self {
        let res = sqlx::query!(
            "SELECT start_date, offset FROM dates WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await;

        match res {
            Ok(res) => Self {
                start_date: res.start_date,
                offset: res.offset,
            },
            Err(e) => match e {
                sqlx::Error::RowNotFound => Self::set_default(pool, user_id).await,
                _ => todo!(),
            },
        }
    }
}
