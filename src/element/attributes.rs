use std::fmt;

#[derive(Copy, Clone)]
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
    rgba: (u8, u8, u8, u8),
    name: ColorName,
}

#[derive(Copy, Clone)]
enum _ColorType {
    RGBA,
    Name,
}

#[derive(Copy, Clone)]
pub struct Color {
    _value: _Color,
    _value_type: _ColorType,
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Color {
            _value: _Color { rgba: (r, g, b, a) },
            _value_type: _ColorType::RGBA,
        };
    }

    pub fn from_name(name: ColorName) -> Self {
        return Color {
            _value: _Color { name },
            _value_type: _ColorType::Name,
        };
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self._value_type {
            _ColorType::Name => unsafe { self._value.name.to_string() },
            _ColorType::RGBA => unsafe {
                format!(
                    "{} {} {} {}",
                    self._value.rgba.0, self._value.rgba.1, self._value.rgba.2, self._value.rgba.3
                )
            },
        };
        write!(f, "{}", value)
    }
}

#[derive(Copy, Clone)]
pub struct Gradient {/*TODO implement*/}

#[derive(Copy, Clone)]
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

impl PaintServer {
    pub fn from_gradient(gradient: Gradient) -> Self {
        return PaintServer {
            _value: _PaintServer { gradient },
            _value_type: _PaintServerType::Gradient,
        };
    }
    pub fn from_pattern(pattern: Pattern) -> Self {
        return PaintServer {
            _value: _PaintServer { pattern },
            _value_type: _PaintServerType::Pattern,
        };
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

union _Paint {
    color: Color,
    paint_server: PaintServer,
}

enum _PaintType {
    Color,
    PaintServer,
}

pub struct Paint {
    _value: _Paint,
    _value_type: _PaintType,
}

impl Paint {
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
            _PaintType::Color => unsafe { self._value.color.to_string() },
            _PaintType::PaintServer => unsafe {
                unimplemented!("Need to implement gradient and pattern");
            },
        };
        write!(f, "{}", value)
    }
}

pub struct Attributes {
    pub stroke: Paint,
}