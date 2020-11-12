use crate::element::attributes::Attributes;
use std::collections::HashMap;

pub mod attributes;

pub trait Element {
    fn get_mut_attributes(&mut self) -> &mut Attributes;
    fn get_attributes(&self) -> &Attributes;
    fn tag_name(&self) -> String;
    fn tag_content(&self) -> Option<String> {
        None
    }

    // shared behaviour do not override following methods
    fn render(&self) -> String {
        let mut formatted_attributes = self.get_attributes().to_string();

        if let Some(tag_content) = self.tag_content() {
            format!(
                "<{tag_name} {attributes}>\n{content}\n</{tag_name}>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes,
                content = tag_content,
            )
        } else {
            format!(
                "<{tag_name} {attributes}/>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes,
            )
        }
    }
}
