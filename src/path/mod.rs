use crate::element::Element;
use std::collections::HashMap;

pub mod command;

pub struct Path {
    pub name: String,
    pub commands: Vec<Box<dyn command::Command>>,
    attributes: HashMap<String, String>,
}

impl Path {
    pub fn new(name: &str) -> Path {
        Path {
            name: name.to_string(),
            commands: Vec::new(),
            attributes: HashMap::new(),
        }
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn add_commands(mut self, commands: &mut Vec<Box<dyn command::Command>>) -> Self {
        self.commands.append(commands);
        self
    }
    fn _get_serialized_commands(&self) -> String {
        self.commands
            .iter()
            .map(|cmd| cmd.svg_repr())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Element for Path {
    fn get_mut_attributes(&mut self) -> &mut HashMap<String, String> {
        self.attributes
            .insert("d".to_string(), self._get_serialized_commands());
        self.attributes.insert("id".to_string(), self.name.clone());
        return &mut self.attributes;
    }
    fn get_attributes(&self) -> HashMap<String, String> {
        let mut attributes = self.attributes.clone();
        attributes.insert("d".to_string(), self._get_serialized_commands());
        attributes.insert("id".to_string(), self.name.clone());
        return attributes;
    }
    fn tag_name(&self) -> String {
        "path".to_string()
    }
}
