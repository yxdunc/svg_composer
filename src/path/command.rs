//      CubicBezierCurve()
//      QuadraticBezierCurve()
//      EllipticalCurve()
//      ClosePath()

pub enum CoordinateType {
    Absolute,
    Relative,
}

pub enum LineToOption {
    Default,
    Vertical,
    Horizontal,
}

pub trait Command {
    fn svg_repr(&self) -> String;
}

pub struct MoveTo {
    pub x: f64,
    pub y: f64,
    pub coordinate_type: CoordinateType,
}

impl Command for MoveTo {
    fn svg_repr(&self) -> String {
        let cmd_letter = match self.coordinate_type {
            CoordinateType::Absolute => 'M',
            CoordinateType::Relative => 'm',
        };
        format!("{}{} {}", cmd_letter, self.x, self.y)
    }
}

pub struct LineTo {
    pub x: f64,
    pub y: f64,
    pub option: LineToOption,
    pub coordinate_type: CoordinateType,
}

impl Command for LineTo {
    fn svg_repr(&self) -> String {
        let cmd_letter: char = match self.option {
            LineToOption::Default => 'l',
            LineToOption::Horizontal => 'h',
            LineToOption::Vertical => 'v',
        };

        let cmd_letter: char = match self.coordinate_type {
            CoordinateType::Absolute => cmd_letter.to_ascii_uppercase(),
            CoordinateType::Relative => cmd_letter,
        };

        format!("{}{} {}", cmd_letter, self.x, self.y)
    }
}

//pub struct CubicBezierCurve {
//    pub coordinates_triplets: ((f64, f64), (f64, f64), (f64, f64)),
//    pub coordinate_type: CoordinateType,
//}
//
//impl Command for CubicBezierCurve {
//    fn svg_repr(&self) -> String {
//        let cmd_letter: char = match self.option {
//            LineToOption::Default => 'l',
//            LineToOption::Horizontal => 'h',
//            LineToOption::Vertical => 'v',
//        };
//
//        let cmd_letter: char = if self.coordinate_type == CoordinateType::Absolute {
//            'L'
//        } else {
//            'l'
//        };
//        format!("{}{} {}", cmd_letter, self.x, self.y)
//    }
//}
