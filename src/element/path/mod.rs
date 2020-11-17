use crate::element::attributes::Attributes;
pub use crate::element::path::command::{Command, Commands};
use crate::element::Element;

pub mod command;

pub struct Path {
    attributes: Attributes,
}

/// SVG path element
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path
///
/// # Examples
///
/// ```
/// use svg_composer::element::Path;
/// use svg_composer::element::path::command::{LineTo, CoordinateType, LineToOption, MoveTo, End};
/// use svg_composer::element::path::Command;
///
/// let path = Path::new().set_name("my_path").add_commands(vec![
///     Box::new(MoveTo{
///         point: (0.0, 0.0),
///         coordinate_type: CoordinateType::Absolute,
///     }),
///     Box::new(LineTo{
///         point:(10.0, 10.0),
///         option: LineToOption::Default,
///         coordinate_type: CoordinateType::Absolute,
///     }),
///     Box::new(End{}),
/// ]);
/// ```
impl Path {
    pub fn new() -> Path {
        let mut attributes = Attributes::default();
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
    pub fn add_command(mut self, mut command: Box<dyn command::Command>) -> Self {
        match self.attributes.d {
            Some(ref mut x) => {
                x.commands.push(command);
            }
            None => {
                self.attributes.d = Some(Commands {
                    commands: vec![command],
                });
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
