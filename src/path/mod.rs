use crate::element::attributes::Attributes;
use crate::element::Element;
use crate::path::command::{Command, Commands};

pub mod command;

pub struct Path {
    attributes: Attributes,
}

impl Path {
    pub fn new(name: &str) -> Path {
        let mut attributes = Attributes::default();
        attributes.id = Some(name.to_string());
        attributes.d = Some(Commands {
            commands: Vec::<Box<dyn Command>>::new(),
        });
        Path { attributes }
    }
    pub fn set_name(mut self, name: &str) -> Self {
        self.attributes.id = Some(name.to_string());
        self
    }
    pub fn add_commands(mut self, mut commands: Vec<Box<dyn command::Command>>) -> Self {
        match self.attributes.d {
            Some(ref mut x) => {
                x.commands.append(&mut commands);
            }
            None => {
                self.attributes.d = Some(Commands { commands });
            }
        }
        self
    }
}

impl Element for Path {
    fn get_mut_attributes(&mut self) -> &mut Attributes {
        return &mut self.attributes;
    }
    fn get_attributes(&self) -> &Attributes {
        return &self.attributes;
    }
    fn tag_name(&self) -> String {
        "path".to_string()
    }
}
