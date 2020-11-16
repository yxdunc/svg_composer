use svg_composer::document::Document;
use svg_composer::element::attributes::{Color, ColorName, Paint, StrokeWidth};
use svg_composer::element::path::command::{CoordinateType, End, LineTo, LineToOption, MoveTo};
use svg_composer::element::{Element, Path};

fn main() {
    let svg_document = Document::new(
        vec![Box::new(
            Path::new()
                .set_stroke(Paint::from_color(Color::from_name(ColorName::Fuchsia)))
                .set_stroke_width(StrokeWidth::from_length(10.0))
                .add_commands(vec![
                    Box::new(MoveTo {
                        point: (11., 1.),
                        coordinate_type: CoordinateType::Absolute,
                    }),
                    Box::new(LineTo {
                        point: (10.0, 20.0),
                        option: LineToOption::Default,
                        coordinate_type: CoordinateType::Relative,
                    }),
                    Box::new(LineTo {
                        point: (-20.0, 0.0),
                        option: LineToOption::Default,
                        coordinate_type: CoordinateType::Relative,
                    }),
                    Box::new(End {}),
                ]),
        )],
        Some([0., 0., 100., 100.]),
    );
    println!("{}", svg_document.render());
}
