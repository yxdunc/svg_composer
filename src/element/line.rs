use crate::element::attributes::{Attributes, Size, ToSize};
use crate::element::Element;

#[derive(Clone)]
pub struct Line {
    attributes: Attributes,
}

impl Line {
    pub fn new() -> Self {
        Line {
            attributes: Attributes::default(),
        }
    }
    pub fn set_point_1<I>(mut self, p: (I, I)) -> Self
    where
        I: ToSize,
    {
        self.attributes.x1 = Some(p.0.to_size());
        self.attributes.y1 = Some(p.1.to_size());
        self
    }
    pub fn set_point_2<I>(mut self, p: (I, I)) -> Self
    where
        I: ToSize,
    {
        self.attributes.x2 = Some(p.0.to_size());
        self.attributes.y2 = Some(p.1.to_size());
        self
    }
}

impl Element for Line {
    fn get_mut_attributes(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }

    fn tag_name(&self) -> String {
        "line".to_string()
    }
}
