use peritext::{Behavior, Expand, Style};
use serde_json::json;
use strum::AsRefStr;

use self::citation::Citation;
pub mod citation;

pub trait Formattable: Into<Style> {
    fn tag(&self) -> &'static str;

    // /// Apply a given formatting to the specified range of text.
    // fn apply_formatting(&mut self, range: Range<usize>, formatting: Formatting);

    // /// Remove a given formatting from the specified range of text.
    // fn remove_formatting(&mut self, range: Range<usize>, formatting: Formatting);

    // /// Get the formattings applied to the text at the given position.
    // fn formattings_at(&self, position: usize) -> Vec<Formatting>;

    // /// Get the formattings applied to the text within the given range.
    // fn formattings_in_range(&self, range: Range<usize>) -> Vec<(Range<usize>, Formatting)>;
}

#[derive(
    strum::Display,
    // strum::EnumString,
    // strum::EnumIter,
    Clone,
    PartialEq,
    Eq,
    Debug,
    strum::EnumMessage,
    strum::IntoStaticStr,
)]
pub enum Formatting {
    Bold,
    NotBold,
    // Italic,
    // NotItalic,
    // Underline,
    // NotUnderline,
    // StrikeThrough,
    // NotStrikeThrough,
    // FontSize(u32),
    // FontColor(u8, u8, u8), // RGB
    Link { url: String },
    NotLink,
    Citation(Citation),
    Comment(String),
}

impl From<Formatting> for Style {
    fn from(value: Formatting) -> Self {
        let tag = value.tag();
        match value {
            Formatting::Bold => Style {
                expand: Expand::After,
                behavior: Behavior::Merge,
                type_: tag.into(),
                value: serde_json::Value::Null,
            },
            Formatting::NotBold => Style {
                expand: Expand::After,
                behavior: Behavior::Delete,
                type_: tag.into(),
                value: serde_json::Value::Null,
            },
            Formatting::Link { url } => Style {
                expand: Expand::None,
                behavior: Behavior::Merge,
                type_: tag.into(),
                value: serde_json::Value::String(url),
            },
            Formatting::NotLink => Style {
                expand: Expand::Both,
                behavior: Behavior::Delete,
                type_: tag.into(),
                value: serde_json::Value::Null,
            },
            Formatting::Comment(comment) => Style {
                expand: Expand::None,
                behavior: Behavior::AllowMultiple,
                type_: tag.into(),
                value: serde_json::Value::String(comment.to_owned()),
            },
            Formatting::Citation(citation) => Style {
                expand: Expand::None,
                behavior: Behavior::AllowMultiple,
                type_: tag.into(),
                value: json!(citation),
            },
        }
    }
}

impl Formattable for Formatting {
    fn tag(&self) -> &'static str {
        self.into()
    }
}
