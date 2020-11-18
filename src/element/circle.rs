use crate::element::attributes::Attributes;
use crate::element::Element;

pub struct Circle {
    attributes: Attributes,
}

impl Circle {
    fn new() -> Self {
        Circle {
            attributes: Attributes::default(),
        }
    }
    fn set_pos(mut self, pos: (f64, f64)) -> Self {
        self.attributes.cx = Some(pos.0);
        self.attributes.cy = Some(pos.1);
        self
    }
    fn set_radius(mut self, pos: (f64, f64)) -> Self {
        self.attributes.cx = Some(pos.0);
        self.attributes.cy = Some(pos.1);
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