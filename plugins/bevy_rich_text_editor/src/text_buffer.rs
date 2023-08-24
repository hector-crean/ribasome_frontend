use std::{borrow::Cow, cell::RefCell, ops::RangeBounds};

use crate::text_edit::TextBuffer;
use peritext::{
    rich_text::{IndexType, Span},
    RichText as RichTextInner,
};

use crate::formatting::Formatting;

#[derive(thiserror::Error, Debug)]
enum RichTextError {
    #[error("index {target_index:?} is out of range 0 - {highest_index:?}")]
    IndexOutOfRange {
        target_index: usize,
        highest_index: usize,
    },
}

pub struct RichText {
    inner: RichTextInner,
}

impl RichText {
    pub fn new(id: u64) -> Self {
        let mut text = RichTextInner::new(id);
        text.set_event_index_type(IndexType::Utf16);
        Self { inner: text }
    }
    pub fn id(&self) -> u64 {
        self.inner.id()
    }

    pub fn insert(&mut self, index: usize, text: &str) -> Result<(), RichTextError> {
        if index > self.inner.len() {
            return Err(RichTextError::IndexOutOfRange {
                target_index: index,
                highest_index: self.inner.len(),
            });
        }

        self.inner.insert_utf16(index, text);
        Ok(())
    }
    pub fn delete(&mut self, range: impl RangeBounds<usize>) -> Result<(), RichTextError> {
        // if range > self.inner.len() {
        //     return Err(RichTextError::IndexOutOfRange {
        //         target_index: index + length,
        //         highest_index: self.inner.len(),
        //     });
        // }

        self.inner.delete_utf16(range);
        Ok(())
    }

    pub fn slice<'a>(&'a self, range: impl RangeBounds<usize>) -> Cow<'_, str> {
        self.inner.slice_str(range, IndexType::Utf16).into()
    }

    pub fn annotate(&mut self, range: impl RangeBounds<usize>, formatting: Formatting) {
        self.inner.annotate(range, formatting.into());
    }

    pub fn get_spans(&self) -> Vec<Span> {
        self.inner.get_spans()
    }
}

impl TextBuffer for RichText {
    fn is_mutable(&self) -> bool {
        true
    }
    fn slice(&self) -> Cow<str> {
        self.inner.to_string().into()
    }
    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.insert(char_index, text);
        text.len()
    }
    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.delete(char_range);
    }
}
