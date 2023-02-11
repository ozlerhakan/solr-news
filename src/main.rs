extern crate minidom;

const ENTRY_NS: &'static str = "entry";

#[derive(Debug)]
pub struct News {
    pub title: String,
    pub summary: String,
    pub date: String,
}

fn main() -> Result<(), reqwest::Error> {
    let mut res = match reqwest::blocking::get("https://solr.apache.org/feeds/solr/news.atom.xml") {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let root: minidom::Element = res.text()?.parse().unwrap();

    root.children()
        .map(|child| {
            let title = child.get_child("title", ENTRY_NS).unwrap().text();
            let date = child.get_child("published", ENTRY_NS).unwrap().text();
            let summary = child.get_child("summary", ENTRY_NS).unwrap().text();
            News {
                title: title,
                date: date,
                summary: summary,
            };
        })
        .take(4)
        .for_each(|news| println!("{:?}", news));
    Ok(())
}
