use crate::element::attributes::{Attributes, Size, ToSize};
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
    pub fn set_pos<I>(mut self, pos: (I, I)) -> Self
    where
        I: ToSize,
    {
        self.attributes.x = Some(pos.0.to_size());
        self.attributes.y = Some(pos.1.to_size());
        self
    }
    pub fn set_size<I>(mut self, width: I, height: I) -> Self
    where
        I: ToSize,
    {
        self.attributes.width = Some(width.to_size());
        self.attributes.height = Some(height.to_size());
        self
    }
    pub fn set_rounding<I>(mut self, rx: I, ry: I) -> Self
    where
        I: ToSize,
    {
        self.attributes.rx = Some(rx.to_size());
        self.attributes.ry = Some(ry.to_size());
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
