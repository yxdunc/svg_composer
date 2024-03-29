use crate::element::path::command::Commands;
use log::warn;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::Chars;

#[derive(Copy, Clone, Debug)]
pub enum ColorName {
    Aqua,
    Black,
    Blue,
    Fuchsia,
    Gray,
    Green,
    Lime,
    Maroon,
    Navy,
    Olive,
    Purple,
    Red,
    Silver,
    Teal,
    White,
    Yellow,
}

impl fmt::Display for ColorName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match *self {
            ColorName::Aqua => "aqua",
            ColorName::Black => "black",
            ColorName::Blue => "blue",
            ColorName::Fuchsia => "fuchsia",
            ColorName::Gray => "gray",
            ColorName::Green => "green",
            ColorName::Lime => "lime",
            ColorName::Maroon => "maroon",
            ColorName::Navy => "navy",
            ColorName::Olive => "olive",
            ColorName::Purple => "purple",
            ColorName::Red => "red",
            ColorName::Silver => "silver",
            ColorName::Teal => "teal",
            ColorName::White => "white",
            ColorName::Yellow => "yellow",
        };
        write!(f, "{}", color)
    }
}

#[derive(Copy, Clone)]
union _Color {
    rgb: (u8, u8, u8),
    rgba: (u8, u8, u8, u8),
    name: ColorName,
}

#[derive(Copy, Clone)]
enum _ColorType {
    RGB,
    RGBA,
    Name,
}

#[derive(Copy, Clone)]
pub struct Color {
    _value: _Color,
    _value_type: _ColorType,
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            _value: _Color { rgb: (r, g, b) },
            _value_type: _ColorType::RGB,
        }
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            _value: _Color { rgba: (r, g, b, a) },
            _value_type: _ColorType::RGBA,
        }
    }

    pub fn from_name(name: ColorName) -> Self {
        Color {
            _value: _Color { name },
            _value_type: _ColorType::Name,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self._value_type {
            _ColorType::Name => unsafe { self._value.name.to_string() },
            _ColorType::RGB => unsafe {
                format!(
                    "rgb({},{},{})",
                    self._value.rgba.0, self._value.rgba.1, self._value.rgba.2
                )
            },
            _ColorType::RGBA => unsafe {
                format!(
                    "rgba({},{},{},{})",
                    self._value.rgba.0, self._value.rgba.1, self._value.rgba.2, self._value.rgba.3
                )
            },
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone)]
pub struct ClassName {
    _value: String,
}

impl ClassName {
    pub fn from_string(str: String) -> Result<Self, String> {
        if str.len() == 0 {
            return Err("Empty strings do not conform with css class name standard".to_string());
        }
        let mut str_chars = str.chars();
        Self::_is_first_char_valid(str_chars.next().unwrap())?;
        Self::_are_following_chars_valid(str_chars)?;

        Ok(ClassName { _value: str })
    }
    fn _is_first_char_valid(first_char: char) -> Result<(), String> {
        match first_char {
            'a'..='z' => Ok(()),
            'A'..='Z' => Ok(()),
            '-' | '_' => Ok(()),
            _ => Err("First char does not conform with css class name standard".to_string()),
        }
    }
    fn _are_following_chars_valid(chars: Chars) -> Result<(), String> {
        let mut i = 1;

        for c in chars {
            match c {
                'a'..='z' => (),
                'A'..='Z' => (),
                '-' | '_' => (),
                '0'..='9' => (),
                _ => {
                    return Err(format!(
                        "Char number {} does not conform with css class name standard",
                        i
                    ))
                }
            }
            i += 1;
        }
        Ok(())
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self._value)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Gradient {/*TODO implement*/}

#[derive(Copy, Clone, Debug)]
pub struct Pattern {/*TODO implement*/}

#[derive(Copy, Clone)]
union _PaintServer {
    gradient: Gradient,
    pattern: Pattern,
}

#[derive(Copy, Clone)]
enum _PaintServerType {
    Gradient,
    Pattern,
}

#[derive(Copy, Clone)]
pub struct PaintServer {
    _value: _PaintServer,
    _value_type: _PaintServerType,
}

impl Debug for PaintServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PaintServer {
    pub fn from_gradient(gradient: Gradient) -> Self {
        PaintServer {
            _value: _PaintServer { gradient },
            _value_type: _PaintServerType::Gradient,
        }
    }
    pub fn from_pattern(pattern: Pattern) -> Self {
        PaintServer {
            _value: _PaintServer { pattern },
            _value_type: _PaintServerType::Pattern,
        }
    }
}

impl fmt::Display for PaintServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self._value_type {
            _PaintServerType::Pattern => unimplemented!("Need to implement pattern"),
            _PaintServerType::Gradient => unimplemented!("Need to implement gradient"),
        };
        // TODO uncomment once gradient and pattern are implemented
        // write!(f, "{}", value)
    }
}

#[derive(Copy, Clone)]
union _Paint {
    color: Color,
    paint_server: PaintServer,
    none: (),
}

#[derive(Copy, Clone, Debug)]
enum _PaintType {
    Color,
    PaintServer,
    None,
}

#[derive(Copy, Clone)]
pub struct Paint {
    _value: _Paint,
    _value_type: _PaintType,
}

impl Debug for Paint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Paint {
    pub fn new_empty() -> Self {
        Paint {
            _value: _Paint { none: () },
            _value_type: _PaintType::None,
        }
    }
    pub fn from_color(color: Color) -> Self {
        return Paint {
            _value: _Paint { color },
            _value_type: _PaintType::Color,
        };
    }
    pub fn from_paint_server(paint_server: PaintServer) -> Self {
        unimplemented!("Need to implement gradient and pattern");
    }
}

impl fmt::Display for Paint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self._value_type {
            _PaintType::None => unsafe { "none".to_string() },
            _PaintType::Color => unsafe { self._value.color.to_string() },
            _PaintType::PaintServer => unsafe {
                unimplemented!("Need to implement gradient and pattern");
            },
        };
        write!(f, "{}", value)
    }
}

#[derive(Copy, Clone)]
enum _NumberType {
    Ratio,
    Length,
}

pub trait ToSize {
    fn to_size(&self) -> Size;
}

#[derive(Copy, Clone)]
pub struct Size {
    _value: f64,
    _value_type: _NumberType,
}

impl ToSize for f64 {
    fn to_size(&self) -> Size {
        Size::from_length(*self)
    }
}

impl ToSize for Size {
    fn to_size(&self) -> Size {
        *self
    }
}

impl Size {
    pub fn from_percentage(p: f64) -> Self {
        Size {
            _value: p / 100.0,
            _value_type: _NumberType::Ratio,
        }
    }

    pub fn from_ratio(r: f64) -> Self {
        Size {
            _value: r,
            _value_type: _NumberType::Ratio,
        }
    }

    pub fn from_length(l: f64) -> Self {
        if l < 0.0 {
            warn!("Using a negative number to define a Size")
        }
        Size {
            _value: l,
            _value_type: _NumberType::Length,
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self._value_type {
            _NumberType::Ratio => unsafe { format!("{}%", self._value * 100.0) },
            _NumberType::Length => unsafe { self._value.to_string() },
        };
        write!(f, "{}", value)
    }
}

#[derive(Copy, Clone)]
pub enum StrokeLineCap {
    Round,
    Butt,
    Square,
}

impl fmt::Display for StrokeLineCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_cap = match *self {
            StrokeLineCap::Round => "round",
            StrokeLineCap::Butt => "butt",
            StrokeLineCap::Square => "square",
        };
        write!(f, "{}", line_cap)
    }
}

#[derive(Copy, Clone)]
pub enum LengthAdjust {
    Spacing,
    SpacingAndGlyphs,
}

impl fmt::Display for LengthAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_cap = match *self {
            LengthAdjust::Spacing => "spacing",
            LengthAdjust::SpacingAndGlyphs => "spacingAndGlyphs",
        };
        write!(f, "{}", line_cap)
    }
}

/// A container for attributes of any SVG element
/// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
#[derive(Default, Clone)]
pub struct Attributes {
    // All elements
    pub id: Option<String>,
    pub class: Option<Vec<ClassName>>,
    pub stroke: Option<Paint>,
    pub stroke_width: Option<Size>,
    pub stroke_linecap: Option<StrokeLineCap>,
    pub fill: Option<Paint>,

    // Path
    pub d: Option<Commands>,

    // Rectangle
    pub width: Option<Size>,
    pub height: Option<Size>,
    pub rx: Option<Size>,
    pub ry: Option<Size>,
    pub x: Option<Size>,
    pub y: Option<Size>,

    // Line
    pub x1: Option<Size>,
    pub y1: Option<Size>,
    pub x2: Option<Size>,
    pub y2: Option<Size>,

    // Circle
    pub cx: Option<Size>,
    pub cy: Option<Size>,
    pub radius: Option<Size>,

    // Text
    pub text_length: Option<Size>,
    pub length_adjust: Option<LengthAdjust>,
    pub rotate_chars: Option<Vec<f64>>,
    pub dx: Option<Size>,
    pub dy: Option<Size>,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let test = self.id.as_ref().and_then(|x| Some(format!("id={}", x)));
        let formatted_attributes: String = vec![
            self.id.as_ref().and_then(|x| Some(format!("id=\"{}\"", x))),
            self.class.as_ref().and_then(|x| {
                Some(format!(
                    "class=\"{}\"",
                    x.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
            }),
            self.stroke
                .as_ref()
                .and_then(|x| Some(format!("stroke=\"{}\"", x))),
            self.stroke_width
                .as_ref()
                .and_then(|x| Some(format!("stroke-width=\"{}\"", x))),
            self.stroke_linecap
                .as_ref()
                .and_then(|x| Some(format!("stroke-linecap=\"{}\"", x))),
            self.fill
                .as_ref()
                .and_then(|x| Some(format!("fill=\"{}\"", x))),
            self.d.as_ref().and_then(|x| Some(format!("d=\"{}\"", x))),
            self.cx.as_ref().and_then(|x| Some(format!("cx=\"{}\"", x))),
            self.cy.as_ref().and_then(|x| Some(format!("cy=\"{}\"", x))),
            self.radius
                .as_ref()
                .and_then(|x| Some(format!("r=\"{}\"", x))),
            self.x.as_ref().and_then(|x| Some(format!("x=\"{}\"", x))),
            self.y.as_ref().and_then(|x| Some(format!("y=\"{}\"", x))),
            self.x1.as_ref().and_then(|x| Some(format!("x1=\"{}\"", x))),
            self.y1.as_ref().and_then(|x| Some(format!("y1=\"{}\"", x))),
            self.x2.as_ref().and_then(|x| Some(format!("x2=\"{}\"", x))),
            self.y2.as_ref().and_then(|x| Some(format!("y2=\"{}\"", x))),
            self.rx.as_ref().and_then(|x| Some(format!("rx=\"{}\"", x))),
            self.ry.as_ref().and_then(|x| Some(format!("ry=\"{}\"", x))),
            self.dx.as_ref().and_then(|x| Some(format!("dx=\"{}\"", x))),
            self.dy.as_ref().and_then(|x| Some(format!("dy=\"{}\"", x))),
            self.width
                .as_ref()
                .and_then(|x| Some(format!("width=\"{}\"", x))),
            self.height
                .as_ref()
                .and_then(|x| Some(format!("height=\"{}\"", x))),
            self.text_length
                .as_ref()
                .and_then(|x| Some(format!("textLength=\"{}\"", x))),
            self.length_adjust
                .as_ref()
                .and_then(|x| Some(format!("lengthAdjust=\"{}\"", x))),
            self.rotate_chars.as_ref().and_then(|x| {
                Some(format!(
                    "rotate=\"{}\"",
                    x.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                ))
            }),
        ]
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.clone().unwrap())
        .collect::<Vec<String>>()
        .join(" ");
        write!(f, "{}", formatted_attributes)
    }
}
