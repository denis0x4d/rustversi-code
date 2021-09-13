use core::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OccupyType {
    Black,
    White,
    Empty,
}

impl OccupyType {
    pub fn get_opposite_type(t:OccupyType) -> OccupyType {
        return match t {
            OccupyType::Black => OccupyType::White,
            OccupyType::White => OccupyType::Black,
            OccupyType::Empty => { panic!("No opposite for Empty type");}
        };
    }


    pub fn from_string(s:&String) -> OccupyType {
        return match (*s).as_str() {
            "O" => OccupyType::White,
            "#" => OccupyType::Black,
            _ => OccupyType::Empty
        };
    }


    pub fn to_string(&self) -> String {
        return match self {
            OccupyType::Black => { "#".to_string() }
            OccupyType::White => { "O".to_string() }
            OccupyType::Empty => { " ".to_string() }
        };
    }
}

impl fmt::Display for OccupyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[test]
fn occupy_print_test() {
    let p = OccupyType::Black;
    assert_eq!("#".to_string(), p.to_string());
    assert_eq!("#".to_string(), format!("{}", p));
}

#[test]
fn occupy_opposite_test() {
    assert_eq!("O".to_string(), OccupyType::get_opposite_type(OccupyType::Black).to_string());
}
