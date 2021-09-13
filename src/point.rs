use core::fmt;


pub const BOARD_SIZE: u8 = 8;
pub const C11: Point = Point{x:1,y:1};
pub const C18: Point = Point{x:1, y:BOARD_SIZE};
pub const C81: Point = Point{x:BOARD_SIZE, y:1};
pub const C88: Point = Point{x:BOARD_SIZE, y:BOARD_SIZE};

#[derive(Debug, Hash, Copy, Clone, PartialEq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn check_point(x: i8, y: i8) -> bool {
        return x >= 1 && x <= (BOARD_SIZE as i8) && y >=1  && y <= (BOARD_SIZE as i8);
    }
    pub fn to_string(&self) -> String {
        format!("({},{})", self.x, self.y)
    }

    pub fn new(x: u8, y: u8) -> Self {
        //println!("{:?}{:?}",x,y);
        if x <= 0 || x > BOARD_SIZE ||
            y <= 0 || y > BOARD_SIZE {
            panic!("Parameters out of boundaries");
        }
        Point { x, y }
    }
    pub fn x(&self) -> u8 {
       return self.x;
    }
    pub fn y(&self) -> u8 {
        return self.y;
    }

    pub fn is_corner(self)->bool{
        return self == C11 || self == C18 || self == C81 || self == C88;
    }

//    //WASM fix
//    pub fn is_corner(self)->bool{
//        return (self.x == 1 && self.y == 1) || 
//               (self.x == 1 && self.y == 8) || 
//               (self.x == 8 && self.y == 1) || 
//               (self.x == 8 && self.y == 8);
//    }
}

impl Eq for Point {}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn point_test() {
    let p = Point::new(4,2);
    assert_eq!("(4,2)", p.to_string());
    assert_eq!("(4,2)", format!("{}", p));
    assert_eq!(2, p.y());
}
#[test]
fn point_eq_test() {
    let p1 = Point::new(4,2);
    let p2 = &Point::new(4,2);
    assert_eq!(p1, *p2);
}

