use chrono::{DateTime, Utc};
use clap::Parser;
use minidom::{Element, Node};
extern crate minidom;

const ENTRY_NS: &'static str = "http://www.w3.org/2005/Atom";
const SOLR_ATOM: &'static str = "https://solr.apache.org/feeds/solr/news.atom.xml";

#[derive(Debug)]
struct News {
    title: String,
    summary: String,
    date: String,
}

#[derive(Parser, Debug)]
#[command(name = "solrnews")]
#[command(bin_name = "solrnews")]
#[command(version = "0.0.1")]
#[command(author, version, about, long_about = None)]
struct SolrNewsArgs {
    #[arg(short, long, default_value_t = 5)]
    take: usize,
}

fn main() -> Result<(), reqwest::Error> {
    let args = SolrNewsArgs::parse();

    let res = match reqwest::blocking::get(SOLR_ATOM) {
        Ok(it) => it,
        Err(err) => return Err(err),
    };
    let root: Element = res.text()?.parse().unwrap();

    root.children()
        .filter(|s| s.is("entry", ENTRY_NS))
        .take(args.take)
        .map(|entry| {
            let title = entry
                .get_child("title", entry.ns().as_str())
                .unwrap_or(
                    &Element::builder("title", entry.ns().as_str())
                        .append(Node::Text("Empty".to_owned()))
                        .build(),
                )
                .text();
            let date = entry
                .get_child("published", entry.ns().as_str())
                .unwrap_or(
                    &Element::builder("published", entry.ns().as_str())
                        .append(Node::Text("0000-00-00T00:00:00+00:00".to_owned()))
                        .build(),
                )
                .text()
                .parse::<DateTime<Utc>>()
                .unwrap()
                .format("%d/%m/%Y")
                .to_string();

            let summary: String = entry
                .get_child("summary", entry.ns().as_str())
                .map(|c| c.nodes())
                .unwrap()
                .filter(|e| e.as_text().is_some())
                .map(|node| {
                    return node.as_text().unwrap();
                })
                .nth(1)
                .into_iter()
                .collect();

            let summary = summary_details(summary);
            return News {
                title,
                date,
                summary,
            };
        })
        .enumerate()
        .for_each(|item| {
            println!(
                "{}. {} - {} - {}",
                item.0 + 1,
                item.1.title,
                item.1.date,
                item.1.summary
            );
        });
    Ok(())
}

fn summary_details(summary: String) -> String {
    let mut summary = summary.replace(">", "");
    let tokens = summary.split(' ');
    let mut take = tokens.clone().count();
    if take > 9 {
        take = 8
    }
    let description: Vec<&str> = tokens.take(take).collect();
    summary = description.join(" ");
    if take == 8 {
        summary = summary + "...";
    }
    summary.to_string()
}
