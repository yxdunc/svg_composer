use crate::element::attributes::{Attributes, Size, ToSize};
use crate::element::Element;

#[derive(Clone)]
pub struct Circle {
    attributes: Attributes,
}

impl Circle {
    pub fn new() -> Self {
        Circle {
            attributes: Attributes::default(),
        }
    }
    pub fn set_pos<I>(mut self, pos: (I, I)) -> Self
    where
        I: ToSize,
    {
        self.attributes.cx = Some(pos.0.to_size());
        self.attributes.cy = Some(pos.1.to_size());
        self
    }
    pub fn set_radius<I>(mut self, radius: I) -> Self
    where
        I: ToSize,
    {
        self.attributes.radius = Some(radius.to_size());
        self
    }
}

impl Element for Circle {
    fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn tag_name(&self) -> String {
        "circle".to_string()
    }
}
