use crate::element::attributes::{Attributes, LengthAdjust, Size};
use crate::element::Element;

#[derive(Clone)]
pub struct Text {
    attributes: Attributes,
    value: String,
}

impl Text {
    pub fn new(value: String) -> Self {
        Text {
            attributes: Attributes::default(),
            value,
        }
    }
    pub fn set_pos(mut self, pos: (f64, f64)) -> Self {
        self.attributes.x = Some(pos.0);
        self.attributes.y = Some(pos.1);
        self
    }
    /// Set position relative to previous text element
    pub fn set_relative_pos(mut self, pos: (f64, f64)) -> Self {
        self.attributes.dx = Some(pos.0);
        self.attributes.dy = Some(pos.1);
        self
    }
    /// Set rotation for individual characters
    pub fn set_char_rotation(mut self, list_of_char_rotation: Vec<f64>) -> Self {
        self.attributes.rotate_chars = Some(list_of_char_rotation);
        self
    }
    pub fn set_length(mut self, len: Size) -> Self {
        self.attributes.text_length = Some(len);
        self
    }
    pub fn set_length_adjust(mut self, adjust: LengthAdjust) -> Self {
        self.attributes.length_adjust = Some(adjust);
        self
    }
}

impl Element for Text {
    fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn tag_name(&self) -> String {
        "text".to_string()
    }

    fn tag_content(&self) -> Option<String> {
        Some(self.value.clone())
    }
}
