use crate::Style;

pub mod citation;

pub trait Annotation: Into<Style> {
    fn tag(&self) -> &'static str;
}
