#![allow(unused)]
use brp_web::{
    brp::{
        books::Book,
        model::{UserDates, UserReadings},
    },
    utils::{today_date, today_ymd},
};
use chrono::NaiveDate;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::str::FromStr;

async fn user_reading_example(pool: &SqlitePool) {
    let user_reading = UserReadings::new(
        1,
        vec![
            vec![Book::Matthew, Book::Mark, Book::Luke],
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
            vec![Book::John],
        ],
    );
    user_reading.replace_current(&pool).await;

    let user_readings = UserReadings::from_user(&pool, 1).await;
    println!("{:#?}", user_readings);
}

async fn user_dates_example(pool: &SqlitePool) {
    let (y, m, d) = today_ymd();
    UserDates::set(
        pool,
        1,
        NaiveDate::from_ymd_opt(y, 1, 1).expect("valid year, month, day"),
        0,
    )
    .await;
}

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .connect("./database.sqlite")
        .await
        .unwrap();

    user_dates_example(&pool).await;
    user_reading_example(&pool).await;
}
