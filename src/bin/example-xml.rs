use brp_web::brp::content::Bible;

fn main() {
    let text = std::fs::read_to_string("./chapter.xml").unwrap();

    let b: Bible = serde_xml_rs::from_str(&text).unwrap();
    println!("{:#?}", b);
}
