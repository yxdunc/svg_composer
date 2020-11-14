pub mod document;
pub mod element;

#[cfg(test)]
mod tests {
    use crate::document;
    use crate::element;
    use crate::element::path;
    use crate::element::Element;

    #[test]
    fn should_render_path() {
        let mut path_commands: Vec<Box<dyn path::Command>> = vec![
            Box::new(path::command::MoveTo {
                point: (0_f64, 0_f64),
                coordinate_type: path::command::CoordinateType::Absolute,
            }),
            Box::new(path::command::LineTo {
                point: (10.0, 0.0),
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
            Box::new(path::command::LineTo {
                point: (0.0, 10.0),
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
            Box::new(path::command::LineTo {
                point: (-10.0, 0.0),
                option: path::command::LineToOption::Default,
                coordinate_type: path::command::CoordinateType::Relative,
            }),
        ];
        let paths: Vec<Box<dyn Element>> = vec![Box::new(
            element::path::Path::new()
                .set_name("path_1")
                .add_commands(path_commands),
        )];
        let document = document::Document::new(paths, None);
        assert_eq!(document.render(), "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\">\n<element.path id=\"path_1\" d=\"M0 0 l10 0 l0 10 l-10 0\"/>\n</svg>\n");
    }
}
