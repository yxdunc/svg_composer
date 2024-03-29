use dyn_clone::DynClone;
use std::fmt;

pub trait Command: fmt::Display + DynClone {}
dyn_clone::clone_trait_object!(Command);
impl Command for MoveTo {}
impl Command for LineTo {}
impl Command for CubicBezierCurve {}
impl Command for QuadraticBezierCurve {}
impl Command for Arc {}
impl Command for End {}

#[derive(Clone)]
pub struct Commands {
    pub commands: Vec<Box<dyn Command>>,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.commands
                .iter()
                .map(|cmd| cmd.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Copy, Clone)]
pub enum CoordinateType {
    Absolute,
    Relative,
}

#[derive(Copy, Clone)]
pub enum LineToOption {
    Default,
    Vertical,
    Horizontal,
}

#[derive(Copy, Clone)]
pub struct MoveTo {
    pub point: (f64, f64),
    pub coordinate_type: CoordinateType,
}
impl fmt::Display for MoveTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_letter = match self.coordinate_type {
            CoordinateType::Absolute => 'M',
            CoordinateType::Relative => 'm',
        };
        write!(f, "{}{} {}", cmd_letter, self.point.0, self.point.1)
    }
}

#[derive(Copy, Clone)]
pub struct LineTo {
    pub point: (f64, f64),
    pub option: LineToOption,
    pub coordinate_type: CoordinateType,
}

impl fmt::Display for LineTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_letter: char = match self.option {
            LineToOption::Default => 'l',
            LineToOption::Horizontal => 'h',
            LineToOption::Vertical => 'v',
        };

        let cmd_letter: char = match self.coordinate_type {
            CoordinateType::Absolute => cmd_letter.to_ascii_uppercase(),
            CoordinateType::Relative => cmd_letter,
        };

        match self.option {
            LineToOption::Default => write!(f, "{}{} {}", cmd_letter, self.point.0, self.point.1),
            LineToOption::Horizontal => write!(f, "{}{}", cmd_letter, self.point.0),
            LineToOption::Vertical => write!(f, "{}{}", cmd_letter, self.point.1),
        }
    }
}

#[derive(Copy, Clone)]
pub struct CubicBezierCurve {
    pub point: (f64, f64),
    pub control_point_1: Option<(f64, f64)>,
    pub control_point_2: (f64, f64),
    pub coordinate_type: CoordinateType,
}

impl fmt::Display for CubicBezierCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(control_point_1) = self.control_point_1 {
            let cmd_letter: char = match self.coordinate_type {
                CoordinateType::Absolute => 'C',
                CoordinateType::Relative => 'c',
            };
            write!(
                f,
                "{}{} {} {} {} {} {}",
                cmd_letter,
                control_point_1.0,
                control_point_1.1,
                self.control_point_2.0,
                self.control_point_2.1,
                self.point.0,
                self.point.1,
            )
        } else {
            let cmd_letter: char = match self.coordinate_type {
                CoordinateType::Absolute => 'S',
                CoordinateType::Relative => 's',
            };
            write!(
                f,
                "{}{} {} {} {}",
                cmd_letter,
                self.control_point_2.0,
                self.control_point_2.1,
                self.point.0,
                self.point.1,
            )
        }
    }
}

#[derive(Copy, Clone)]
pub struct QuadraticBezierCurve {
    pub point: (f64, f64),
    pub control_point_1: Option<(f64, f64)>,
    pub coordinate_type: CoordinateType,
}

impl fmt::Display for QuadraticBezierCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(control_point_1) = self.control_point_1 {
            let cmd_letter: char = match self.coordinate_type {
                CoordinateType::Absolute => 'Q',
                CoordinateType::Relative => 'q',
            };
            write!(
                f,
                "{}{} {} {} {}",
                cmd_letter, control_point_1.0, control_point_1.1, self.point.0, self.point.1,
            )
        } else {
            let cmd_letter: char = match self.coordinate_type {
                CoordinateType::Absolute => 'T',
                CoordinateType::Relative => 't',
            };
            write!(f, "{}{} {}", cmd_letter, self.point.0, self.point.1,)
        }
    }
}

#[derive(Copy, Clone)]
pub struct Arc {
    pub radius: (f64, f64),
    pub x_axis_rotation: f64,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
    pub point: (f64, f64),
    pub coordinate_type: CoordinateType,
}

impl Arc {
    pub fn new_circular_ellipse(
        point: (f64, f64),
        radius: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
        coordinate_type: CoordinateType,
    ) -> Self {
        Arc {
            radius: (radius, radius),
            x_axis_rotation: 0.0,
            large_arc_flag,
            sweep_flag,
            point,
            coordinate_type,
        }
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd_letter: char = match self.coordinate_type {
            CoordinateType::Absolute => 'A',
            CoordinateType::Relative => 'a',
        };
        write!(
            f,
            "{}{} {} {} {} {} {} {}",
            cmd_letter,
            self.radius.0,
            self.radius.1,
            self.x_axis_rotation,
            self.large_arc_flag as i8,
            self.sweep_flag as i8,
            self.point.0,
            self.point.1,
        )
    }
}

#[derive(Copy, Clone)]
pub struct End {}

impl fmt::Display for End {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 'Z'.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_move_to() {
        let cmd_abs = MoveTo {
            point: (3.14, 42.0),
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_rel = MoveTo {
            point: (3.14, 42.0),
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        assert_eq!(cmd_abs, "M3.14 42");
        assert_eq!(cmd_rel, "m3.14 42");
    }

    #[test]
    fn should_format_line_to() {
        let cmd_abs = LineTo {
            point: (3.14, 42.0),
            option: LineToOption::Default,
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_rel = LineTo {
            point: (3.14, 42.0),
            option: LineToOption::Default,
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        let cmd_vertical = LineTo {
            point: (3.14, 42.0),
            option: LineToOption::Vertical,
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_horizontal = LineTo {
            point: (3.14, 42.0),
            option: LineToOption::Horizontal,
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        assert_eq!(cmd_abs, "L3.14 42");
        assert_eq!(cmd_rel, "l3.14 42");
        assert_eq!(cmd_vertical, "V42");
        assert_eq!(cmd_horizontal, "H3.14");
    }
    #[test]
    fn should_format_cubic_bezier_curve() {
        let cmd_abs = CubicBezierCurve {
            point: (3.14, 42.0),
            control_point_1: None,
            control_point_2: (1.0, 3.0),
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_rel = CubicBezierCurve {
            point: (3.14, 42.0),
            control_point_1: None,
            control_point_2: (1.0, 3.0),
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        let cmd_two_controls = CubicBezierCurve {
            point: (3.14, 42.0),
            control_point_1: Some((4.0, 5.0)),
            control_point_2: (1.0, 3.0),
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        assert_eq!(cmd_abs, "S1 3 3.14 42");
        assert_eq!(cmd_rel, "s1 3 3.14 42");
        assert_eq!(cmd_two_controls, "c4 5 1 3 3.14 42");
    }
    #[test]
    fn should_format_quadratic_bezier_curve() {
        let cmd_abs = QuadraticBezierCurve {
            point: (3.14, 42.0),
            control_point_1: Some((3.0, 4.0)),
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_rel = QuadraticBezierCurve {
            point: (3.14, 42.0),
            control_point_1: Some((3.0, 4.0)),
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        let cmd_no_control = QuadraticBezierCurve {
            point: (3.14, 42.0),
            control_point_1: None,
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        assert_eq!(cmd_abs, "Q3 4 3.14 42");
        assert_eq!(cmd_rel, "q3 4 3.14 42");
        assert_eq!(cmd_no_control, "T3.14 42");
    }
    #[test]
    fn should_format_arc() {
        let cmd_abs = Arc {
            radius: (5.0, 5.0),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: false,
            point: (3.14, 42.0),
            coordinate_type: CoordinateType::Absolute,
        }
        .to_string();
        let cmd_rel = Arc {
            radius: (5.0, 5.0),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: false,
            point: (3.14, 42.0),
            coordinate_type: CoordinateType::Relative,
        }
        .to_string();
        assert_eq!(cmd_abs, "A5 5 0 0 0 3.14 42");
        assert_eq!(cmd_rel, "a5 5 0 0 0 3.14 42");
    }
}
