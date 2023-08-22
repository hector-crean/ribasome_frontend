use crate::{Behavior, Expand, InternalString, RichText, Style};
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde_json::json;

use super::Annotation;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Author {
    Individual(String), // "John Doe"
    Group(String),      // "The XYZ Group"
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct JournalArticle {
    title: String,
    authors: Vec<Author>,
    journal_name: String,
    publication_date: DateTime<Utc>, // This could also be a Date type
    volume: u32,
    issue: Option<u32>,
    page_numbers: (u32, u32), // start and end pages
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Citation {
    Apa(JournalArticle),
    Mla(JournalArticle),
    Chicago(JournalArticle),
    // ... other styles ...
}

impl Citation {
    fn format(&self) -> String {
        match self {
            Citation::Apa(article) => {
                let authors = article
                    .authors
                    .iter()
                    .map(|a| match a {
                        Author::Individual(name) => format!("{}", name),
                        Author::Group(name) => format!("{}", name),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(
                    "{} ({}). {}. {}. {}({}), {}-{}.",
                    authors,
                    article.publication_date,
                    article.title,
                    article.journal_name,
                    article.volume,
                    article.issue.as_ref().unwrap_or(&0),
                    article.page_numbers.0,
                    article.page_numbers.1
                )
            }
            Citation::Mla(article) => {
                // Similar logic but different formatting
                // For simplicity, using a stub string here
                format!("MLA Format for: {}", article.title)
            }
            Citation::Chicago(article) => {
                // Similar logic but different formatting
                // For simplicity, using a stub string here
                format!("Chicago Format for: {}", article.title)
            } // ... other styles ...
        }
    }
}

impl Annotation for Citation {
    fn tag(&self) -> &'static str {
        "citation"
    }
}

impl From<Citation> for Style {
    fn from(value: Citation) -> Self {
        Style {
            expand: Expand::None,
            behavior: Behavior::Merge,
            type_: InternalString::from(value.tag()),
            value: json!(value),
        }
    }
}
