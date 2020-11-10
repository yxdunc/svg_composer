pub mod document;
pub mod element;
pub mod path;

#[cfg(test)]
mod tests {
    use crate::element::Element;
    use crate::{document, path};

    #[test]
    fn should_render_path() {
        let mut path_commands: Vec<Box<dyn path::command::Command>> = vec![
            Box::new(path::command::MoveTo {
                x: 0_f64,
                y: 0_f64,
                coordinate_type: path::command::CoordinateType::Absolute,
            }),
            Box::new(path::command::LineTo {
                x: 10.0,
                y: 0.0,
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
            Box::new(path::command::LineTo {
                x: 0.0,
                y: 10.0,
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
            Box::new(path::command::LineTo {
                x: -10.0,
                y: 0.0,
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
        ];
        let paths: Vec<Box<dyn Element>> = vec![Box::new(
            path::Path::new("path_1").add_commands(&mut path_commands),
        )];
        let document = document::Document::new(paths, None, None, None, None);
        assert_eq!(document.render(), "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\">\n<path d=\"M0 0 l10 0 l0 10 l-10 0\" id=\"path_1\"/>\n</svg>\n");
    }
}
