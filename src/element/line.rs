use crate::element::attributes::{Attributes, Size};
use crate::element::Element;

pub struct Line {
    attributes: Attributes,
}

impl Line {
    pub fn new() -> Self {
        Line {
            attributes: Attributes::default(),
        }
    }
    pub fn set_point_1(mut self, p: (f64, f64)) -> Self {
        self.attributes.x1 = Some(p.0);
        self.attributes.y1 = Some(p.1);
        self
    }
    pub fn set_point_2(mut self, p: (f64, f64)) -> Self {
        self.attributes.x2 = Some(p.0);
        self.attributes.y2 = Some(p.1);
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
