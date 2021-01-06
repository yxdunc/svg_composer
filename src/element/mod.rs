use crate::element::attributes::{Attributes, ClassName, Paint, Size, StrokeLineCap};
use std::collections::HashMap;

use crate::element::path::command::End;
pub use path::Path;
use std::fmt;
use std::fmt::Formatter;

pub mod attributes;
pub mod circle;
pub mod line;
pub mod path;
pub mod rect;
pub mod text;

/// Trait representing a SVG element
/// Struct implementing this trait must not reimplement fmt::Display trait
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Element
pub trait Element {
    /// This method should return a mutable reference to the attribute field stored in the struct
    fn get_mut_attributes(&mut self) -> &mut Attributes;
    /// This method should return a reference to the attribute field stored in the struct
    fn get_attributes(&self) -> &Attributes;
    /// This method should return the name of the element used in the corresponding svg tag
    fn tag_name(&self) -> String;
    /// This method should return content of between the opening and ending tag of the element.
    /// Implemented to return None by default.
    /// If the returned value is None the element will just have a opening tag of the form <xxx/>
    fn tag_content(&self) -> Option<String> {
        None
    }
    fn set_id(mut self, value: &str) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().id = Some(value.to_string());
        self
    }
    fn set_stroke(mut self, value: Paint) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().stroke = Some(value);
        self
    }
    fn set_stroke_width(mut self, value: Size) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().stroke_width = Some(value);
        self
    }
    fn set_stroke_linecap(mut self, value: StrokeLineCap) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().stroke_linecap = Some(value);
        self
    }
    fn set_fill(mut self, value: Paint) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().fill = Some(value);
        self
    }
    fn set_classes(mut self, value: Vec<ClassName>) -> Self
    where
        Self: Sized,
    {
        self.get_mut_attributes().class = Some(value);
        self
    }
}

impl fmt::Display for dyn Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut formatted_attributes = self.get_attributes().to_string();
        if let Some(tag_content) = self.tag_content() {
            write!(
                f,
                "<{tag_name} {attributes}>\n{content}\n</{tag_name}>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes,
                content = tag_content,
            )
        } else {
            write!(
                f,
                "<{tag_name} {attributes}/>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes,
            )
        }
    }
}
