use brp_web::brp::{
    books::Book,
    content::{ChapterDispatcher, IndonesianBible},
};

#[tokio::main]
async fn main() {
    let res = IndonesianBible.get_chapter(&Book::Romans, 1).await;
    println!("{:#?}", res);
}
