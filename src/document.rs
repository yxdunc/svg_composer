use crate::element::Element;

static XMLNS_DEFAULT: &'static str = "http://www.w3.org/2000/svg";
static XMLNS_XLINK_DEFAULT: &'static str = "http://www.w3.org/1999/xlink";

/// Represent an entire SVG document
pub struct Document {
    xmlns: String,
    xmlns_xlink: String,
    view_port: Option<[f32; 2]>,
    view_box: Option<[f32; 4]>,
    elements: Vec<Box<dyn Element>>,
}

impl Document {
    /// Returns a Document
    ///
    /// # Arguments
    ///
    /// * `elements` - The list SVG elements constituting the Document
    /// * `view_box` - The dimensions of the view box
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

    pub fn add_element(&mut self, element: Box<dyn Element>) -> &Self {
        self.elements.push(element);
        self
    }

    pub fn add_elements(&mut self, mut elements: Vec<Box<dyn Element>>) -> &Self {
        self.elements.append(&mut elements);
        self
    }

    pub fn render(&self) -> String {
        let svg_args = vec![
            Some(format!("xmlns=\"{}\"", self.xmlns)),
            Some(format!("xmlns:xlink=\"{}\"", self.xmlns_xlink)),
            self.view_box.and_then(|view_box| {
                Some(format!(
                    "viewBox=\"{}\"",
                    view_box
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
            }),
            self.view_port.and_then(|view_port| {
                Some(format!(
                    "viewPort=\"{}\"",
                    view_port
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
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
