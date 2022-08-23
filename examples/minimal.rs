use svg_composer::element::circle::Circle;
use svg_composer::Document;

fn main() {
    let document = Document::new(
        vec![Box::new(Circle::new().set_radius(10.).set_pos((50., 50.)))],
        Some([0., 0., 100., 100.]),
    );
    let svg_file_content = document.render();

    println!("{}", svg_file_content);
}
