use std::collections::HashMap;

mod attributes;

pub trait Element {
    fn get_mut_attributes(&mut self) -> &mut HashMap<String, String>;
    fn get_attributes(&self) -> HashMap<String, String>;
    fn tag_name(&self) -> String;
    fn tag_content(&self) -> Option<String> {
        None
    }

    // shared behaviour do not override following methods
    fn render(&self) -> String {
        let mut formatted_attributes: Vec<String> = self
            .get_attributes()
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<String>>();

        // Added a sort (and hence mutability) to have predictable output for the tests
        // To remove to improve performances
        formatted_attributes.sort();

        if let Some(tag_content) = self.tag_content() {
            format!(
                "<{tag_name} {attributes}>\n{content}\n</{tag_name}>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes.join(" "),
                content = tag_content,
            )
        } else {
            format!(
                "<{tag_name} {attributes}/>",
                tag_name = self.tag_name(),
                attributes = formatted_attributes.join(" ")
            )
        }
    }
}
