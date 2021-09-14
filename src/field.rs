use std::collections::{HashMap};
use std::fmt;

use crate::point::{Point, BOARD_SIZE};
use crate::occupytype::OccupyType;



#[derive(Debug, Clone)]
pub struct Field {
    data: HashMap<Point, OccupyType>,
}


pub trait FieldAction {
    fn init(&mut self);
    fn get_score_wb(&self) -> (u8, u8);
    fn get_position_wb(&self) -> (Vec<Point>, Vec<Point>);
    fn to_string(&self) -> String;
    fn is_valid_move(&self, p: &Point, bw: OccupyType) -> (bool, u8);
    fn get_list_of_moves(&self, bw: OccupyType) -> Vec<(Point, u8)>;
    fn move_in_game(&mut self, p: &Point, bw: OccupyType) -> u8;
    fn serialize(&self) -> String;
    fn deserialize(&mut self, s: &String);
}


impl Field {
    pub fn new() -> Self {
        let field = Field {
            data: Default::default(),
        };
        field
    }


    pub fn to_string(&self) -> String {
        let mut ret = String::from("\n");
        ret.push_str("   |");
        for i in 1..=BOARD_SIZE {
            ret.push_str(format!(" {:^2}|", i).as_str());
        }
        ret.push_str("\n---|");
        for _ in 1..=BOARD_SIZE {
            ret.push_str("---|");
        }
        ret.push_str("---\n");
        let mut black = 0;
        let mut white = 0;
        for y in (1..=BOARD_SIZE).rev() {
            ret.push_str(format!(" {:^2}|", y).as_str());
            for x in 1..=BOARD_SIZE {
                let p = Point::new(x, y);
                let val = self.get_type(&p);
                if val == OccupyType::Black {
                    black = black + 1;
                } else if val == OccupyType::White {
                    white = white + 1;
                }
                ret.push_str(
                    format!(" {} |",
                            val.to_string()).as_str());
            }

            ret.push_str(format!(" {:^2} \n---|", y).as_str());
            for _ in 1..=BOARD_SIZE {
                ret.push_str("---|");
            }
            ret.push_str("---\n");
        }
        ret.push_str("   |");
        for i in 1..=BOARD_SIZE {
            ret.push_str(format!(" {:^2}|", i).as_str());
        }
        ret.push_str("\n");

        format!("{}\n", ret)
    }

    fn get_type(&self, p: &Point) -> OccupyType {
        return match self.data.get(p) {
            Some(t) => (*t).clone(),
            None => OccupyType::Empty
        };
    }


    fn add(&mut self, p: &Point, bw: OccupyType) -> bool {
        if self.get_type(p) != OccupyType::Empty {
            panic!("Wrong move: Your cannot move to occupied point");
        }
        self.data.insert(*p, bw);
        return true;
    }


    fn change(&mut self, p: &Point, bw: OccupyType) -> bool {
        let point_type = self.get_type(p);
        if point_type == OccupyType::Empty || point_type == bw {
            panic!("Wrong move: Your cannot change empty point or change same color");
        }
        self.data.insert(*p, bw);
        return true;
    }


    #[allow(dead_code)]
    //for debug purposes
    pub(crate) fn setup_field(&mut self, string_field: &str) {
        let lines: Vec<&str> = string_field.split("\n").collect();
        let mut l: u8 = 0;
        let mut first_line = true;
        for s in lines {
            if first_line {
                first_line = false;
                continue;
            }
            for (x, c) in s.chars().enumerate() {
                let y: u8 = BOARD_SIZE - l;
                if c == '*' || c == '#'|| c == 'X' || c == 'x'  {
                    self.add(&Point::new((x + 1) as u8, y), OccupyType::Black);
                } else if c == '0' || c == 'o' || c == 'O' {
                    self.add(&Point::new((x + 1) as u8, y), OccupyType::White);
                }
                if x > BOARD_SIZE as usize {
                    break;
                }
            }
            l = l + 1;
            if l > BOARD_SIZE {
                break;
            }
        }
    }

    fn get_opposite_points(&self, p: &Point, color: OccupyType) -> Vec<Point> {
        let point_type = self.get_type(p);
        if point_type != OccupyType::Empty {
            return vec![];
        }
        let mut result: Vec<Point> = Vec::new();
        let x = p.x();
        let y = p.y();
        let opposite = OccupyType::get_opposite_type(color);
        for dx in -1..=1 {
            for dy in -1..=1 {
                let mut step = 1;
                let mut current_points: Vec<Point> = Vec::new();
                loop {
                    let current_x: i8 = x as i8 + dx * step;
                    let current_y: i8 = y as i8 + dy * step;
                    if !Point::check_point(current_x, current_y) {
                        break;
                    }
                    let current_p = Point::new(current_x as u8, current_y as u8);
                    let current_type = self.get_type(&current_p);

                    if current_type != opposite {
                        if current_type == color {
                            result.append(&mut current_points);
                        }
                        break;
                    }
                    current_points.push(current_p);
                    step = step + 1;
                }
            }
        }
        return result;
    }
}


impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


impl FieldAction for Field {
    fn init(&mut self) {
        let middle = BOARD_SIZE / 2;
        self.add(&Point::new(middle, middle), OccupyType::Black);
        self.add(&Point::new(middle + 1, middle + 1), OccupyType::Black);
        self.add(&Point::new(middle, middle + 1), OccupyType::White);
        self.add(&Point::new(middle + 1, middle), OccupyType::White);
    }


    fn get_score_wb(&self) -> (u8, u8) {
        let mut black = 0;
        let mut white = 0;
        for x in 1..=BOARD_SIZE {
            for y in 1..=BOARD_SIZE {
                let p = Point::new(x, y);
                let val = self.get_type(&p);
                if val == OccupyType::Black {
                    black = black + 1;
                } else if val == OccupyType::White {
                    white = white + 1;
                }
            }
        }
        return (white, black);
    }

    fn get_position_wb(&self) -> (Vec<Point>, Vec<Point>) {
        let mut white: Vec<Point> = Vec::new();
        let mut black: Vec<Point> = Vec::new();

        for key in self.data.keys() {
            let t = self.get_type(key);
            if t == OccupyType::White {
                white.push(*key);
            } else if t == OccupyType::Black {
                black.push(*key);
            } else {
                panic!("Wrong type for point {}", key.to_string());
            }
        };
        return (white, black);
    }

    fn to_string(&self) -> String {
        format!("{}", self.to_string())
    }


    fn is_valid_move(&self, p: &Point, bw: OccupyType) -> (bool, u8) {
        let point_type = self.get_type(p);
        if point_type != OccupyType::Empty {
            return (false, 0);
        }
        let points = self.get_opposite_points(p, bw);
        return (points.len() > 0, points.len() as u8);
    }


    fn get_list_of_moves(&self, bw: OccupyType) -> Vec<(Point, u8)> {
        let mut result: Vec<(Point, u8)> = Vec::new();
        for x in 1..=BOARD_SIZE {
            for y in 1..=BOARD_SIZE {
                let p = Point::new(x, y);
                let point_type = self.get_type(&p);
                if point_type == OccupyType::Empty {
                    let (valid, score) = self.is_valid_move(&p, bw);
                    if valid {
                        result.push((p, score));
                    }
                }
            }
        }
        return result;
    }


    fn move_in_game(&mut self, p: &Point, bw: OccupyType) -> u8 {
        let point_type = self.get_type(p);
        if point_type != OccupyType::Empty {
            panic!("Wrong move to point {}", *p);
        }
        let points = self.get_opposite_points(p, bw);
        let num = points.len() as u8;
        if num == 0 {
            panic!("Invalid move to point {} (no point to change)", *p);
        }
        for current in points {
            self.change(&current, bw);
        }
        self.add(p, bw);
        return num;
    }

    fn serialize(&self) -> String {
        let mut ret = String::from("");

        for y in (1..=BOARD_SIZE).rev() {
            for x in 1..=BOARD_SIZE {
                let p = Point::new(x, y);
                let val = self.get_type(&p);
                ret.push_str(val.to_string().as_str());
            }
        }
        return ret;
    }

    fn deserialize(&mut self, s: &String) {
        if s.len() != (BOARD_SIZE * BOARD_SIZE) as usize {
            panic!("Wrong size of string for the field deserialization")
        }

        let mut i: u8 = 0;
        for (j, c) in s.chars().enumerate() {
            let mut x:u8= (j + 1 - (BOARD_SIZE * i) as usize)as u8;
            if x > BOARD_SIZE  {
                x=1;
                i=i+1;
            }
            let y: u8 = BOARD_SIZE - i;
            self.add(&Point::new(x, y),OccupyType::from_string(&c.to_string()));
        }
    }
}


#[test]
fn field_setup_field_test() {
    let mut f = Field::new();
    f.setup_field(" standard start position


   *
   **
   *o
");
    println!("{}", f.to_string());
    let (w, b) = f.get_score_wb();
    assert_eq!(1, w);
    assert_eq!(4, b);
}

#[test]
fn field_init_field_test() {
    let mut f = Field::new();
    f.init();
    println!("{}", f.to_string());
    let (w, b) = f.get_score_wb();
    assert_eq!(2, w);
    assert_eq!(2, b);
}

#[test]
fn field_get_position_test() {
    let mut f = Field::new();
    f.init();
    println!("{}", f.to_string());
    let (w, b) = f.get_position_wb();
    println!("whites={:?},  blacks={:?}", w, b);
}

#[test]
fn field_is_valid_move1_test() {
    let mut f = Field::new();

    f.setup_field("


   *
   *o
   *o

");
    let p = Point::new(6, 4);
    let (valid, score) = f.is_valid_move(&p, OccupyType::Black);
    println!("{}", f.to_string());
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(2, score);
    assert_eq!(true, valid);
}

#[test]
fn field_is_valid_move2_test() {
    let mut f = Field::new();

    f.setup_field("


   *
   *o
   *o

");
    let p = Point::new(6, 5);
    let (valid, score) = f.is_valid_move(&p, OccupyType::Black);
    println!("{}", f.to_string());
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(true, valid);
    assert_eq!(1, score);
}

#[test]
fn field_is_valid_move3_test() {
    let mut f = Field::new();

    f.setup_field("


   *
   *o
   *o

");
    let p = Point::new(6, 6);
    let (valid, score) = f.is_valid_move(&p, OccupyType::Black);
    println!("{}", f.to_string());
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(true, valid);
    assert_eq!(1, score);
}

#[test]
fn field_is_valid_move4_test() {
    let mut f = Field::new();

    f.setup_field("


   *
   *o
   *o
");
    let p = Point::new(6, 7);
    let (valid, score) = f.is_valid_move(&p, OccupyType::Black);
    println!("{}", f.to_string());
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(false, valid);
}

#[test]
fn field_is_valid_move_edge1_test() {
    let mut f = Field::new();

    f.setup_field("
*******
    *o
    *o

");
    let p = Point::new(8, 8);
    println!("{}", f.to_string());
    let (valid, score) = f.is_valid_move(&p, OccupyType::White);
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(false, valid);
}

#[test]
fn field_is_valid_move_edge2_test() {
    let mut f = Field::new();

    f.setup_field("
*******
     *o
     *o

");
    let p = Point::new(8, 8);
    println!("{}", f.to_string());
    let (valid, score) = f.is_valid_move(&p, OccupyType::Black);
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(true, valid);
}


#[test]
fn field_is_valid_move_edge3_test() {
    let mut f = Field::new();

    f.setup_field("







o******
");
    let p = Point::new(8, 1);
    println!("{}", f.to_string());
    let (valid, score) = f.is_valid_move(&p, OccupyType::White);
    println!("valid={:?},  score={:?}", valid, score);
    assert_eq!(true, valid);
}


#[test]
fn field_get_moves_test() {
    let mut f = Field::new();

    f.setup_field("
*******
     *o
     *o

");
    println!("{}", f.to_string());
    let moves = f.get_list_of_moves(OccupyType::Black);
    println!("valid={:?}", moves);
}

#[test]
fn field_get_change_test() {
    let mut f = Field::new();

    f.setup_field("
*******
     *o
     *o

");
    let p = Point::new(8, 8);
    println!("{}", f.to_string());
    let n = f.move_in_game(&p, OccupyType::Black);
    println!("{}", f.to_string());
    assert_eq!(1, n);
}


#[test]
fn field_serialize_field_test() {
    let mut f = Field::new();
    f.setup_field("
*******
     *o
     *o

");
    let a= f.to_string();
    let s = f.serialize();
    f = Field::new();
    f.deserialize(&s);
    let b=f.to_string();
    assert_eq!(a, b);
}
