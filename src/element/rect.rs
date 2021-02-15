use crate::element::attributes::{Attributes, Size};
use crate::element::Element;

#[derive(Clone)]
pub struct Rectangle {
    attributes: Attributes,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            attributes: Attributes::default(),
        }
    }
    pub fn set_pos(mut self, pos: (f64, f64)) -> Self {
        self.attributes.x = Some(pos.0);
        self.attributes.y = Some(pos.1);
        self
    }
    pub fn set_size(mut self, width: Size, height: Size) -> Self {
        self.attributes.width = Some(width);
        self.attributes.height = Some(height);
        self
    }
    pub fn set_rounding(mut self, rx: f64, ry: f64) -> Self {
        self.attributes.rx = Some(rx);
        self.attributes.ry = Some(ry);
        self
    }
}

impl Element for Rectangle {
    fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn tag_name(&self) -> String {
        "rect".to_string()
    }
}
