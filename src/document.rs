use crate::element::Element;

static XMLNS_DEFAULT: &'static str = "http://www.w3.org/2000/svg";
static XMLNS_XLINK_DEFAULT: &'static str = "http://www.w3.org/1999/xlink";

/// Represent an entire SVG document
#[derive(Clone)]
pub struct Document {
    pub xmlns: String,
    pub xmlns_xlink: String,
    pub view_port: Option<[f32; 2]>,
    pub view_box: Option<[f32; 4]>,
    elements: Vec<Box<dyn Element>>,
}

impl Document {
    /// Returns a Document
    ///
    /// # Arguments
    ///
    /// * `elements` - The list of SVG elements constituting the Document
    /// * `view_box` - The dimensions of the view box (minx, miny, width, height)
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_composer::document::Document;
    /// use svg_composer::element::Element;
    /// let document = Document::new(Vec::<Box<dyn Element>>::new(), None);
    /// ```
    pub fn new(elements: Vec<Box<dyn Element>>, view_box: Option<[f32; 4]>) -> Document {
        Document {
            xmlns: XMLNS_DEFAULT.to_string(),
            xmlns_xlink: XMLNS_XLINK_DEFAULT.to_string(),
            view_box: view_box.or(Some([0.0_f32, 0.0_f32, 100.0_f32, 100.0_f32])),
            view_port: None,
            elements,
        }
    }

    /// Add an SVG Element to the Document
    ///
    /// # Arguments
    ///
    /// * `element` - The SVG element to add to the document
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_composer::document::Document;
    /// use svg_composer::element::circle::Circle;
    /// use svg_composer::element::Element;
    ///
    /// let mut document = Document::new(Vec::<Box<dyn Element>>::new(), None);
    /// document.add_element(Box::new(Circle::new()));
    /// ```
    pub fn add_element(&mut self, element: Box<dyn Element>) -> &Self {
        self.elements.push(element);
        self
    }

    /// Add several SVG Element to the Document
    ///
    /// # Arguments
    ///
    /// * `element` - The list of SVG elements to add to the document
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_composer::document::Document;
    /// use svg_composer::element::circle::Circle;
    /// use svg_composer::element::Element;
    ///
    /// let mut document = Document::new(Vec::<Box<dyn Element>>::new(), None);
    /// document.add_elements(vec![Box::new(Circle::new()), Box::new(Circle::new())]);
    /// ```
    pub fn add_elements(&mut self, mut elements: Vec<Box<dyn Element>>) -> &Self {
        self.elements.append(&mut elements);
        self
    }

    /// Renders the SVG file as a string. This string can then be saved as a regular SVG file.
    ///
    /// # Examples
    ///
    /// ```
    /// use svg_composer::document::Document;
    /// use svg_composer::element::circle::Circle;
    /// use svg_composer::element::Element;
    ///
    /// let mut document = Document::new(vec![Box::new(Circle::new().set_pos((50., 50.)).set_radius(10.))], Some([0., 0., 100., 100.]));
    /// let svg_file_content = document.render();
    /// let expected = "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\">\n<circle cx=\"50\" cy=\"50\" r=\"10\"/>\n</svg>\n";
    ///
    /// assert_eq!(expected, svg_file_content);
    /// ```
    pub fn render(&self) -> String {
        let svg_args = vec![
            Some(format!("xmlns=\"{}\"", self.xmlns)),
            Some(format!("xmlns:xlink=\"{}\"", self.xmlns_xlink)),
            self.view_box.map(|view_box| {
                format!(
                    "viewBox=\"{}\"",
                    view_box
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }),
            self.view_port.map(|view_port| {
                format!(
                    "viewPort=\"{}\"",
                    view_port
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }),
        ]
        .iter()
        .filter(|s| s.is_some())
        .map(|s| s.as_ref().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(" ");

        let elements = self
            .elements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "<svg {svg_args}>\n{elements}</svg>\n",
            svg_args = svg_args,
            elements = if elements.len() != 0 {
                elements + "\n"
            } else {
                "".to_string()
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_simple() {
        let document = Document::new(Vec::<Box<dyn Element>>::new(), None);
        assert_eq!(document.render(), "<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 100 100\">\n</svg>\n");
    }
}
