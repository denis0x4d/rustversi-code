use rustversi::point::{Point};
use rustversi::occupytype::OccupyType;
use rustversi::field::{Field, FieldAction};
use rustversi::game::{computer_move, possible_movement};



fn beginning_choice() -> OccupyType {

    loop {
        let mut line = String::new();
        println!(" O - 1, # - 2 ?");
        std::io::stdin().read_line(&mut line).unwrap();
        let res = line.trim().parse::<u8>();
        if res.is_ok() {
            let sel = res.unwrap();
            if sel == 1 {
                println!("Selected: {}", OccupyType::White);
                return OccupyType::White;
            } else if sel == 2 {
                println!("Selected: {}", OccupyType::Black);
                return OccupyType::Black;
            }
        }
    }
}

fn get_move() -> Point {
    loop {
        let mut line = String::new();

        println!(" x y ?");
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split_ascii_whitespace();

        let x = match iter.next() {
            Some(s) => {
                let res = s.parse::<u8>();
                if res.is_ok() {
                    res.unwrap()
                } else {
                    0
                }
            }
            None => continue
        };
        let y = match iter.next() {
            Some(s) => {
                let res = s.parse::<u8>();
                if res.is_ok() {
                    res.unwrap()
                } else {
                    0
                }
            }
            None => continue
        };

        if Point::check_point(x as i8, y as i8) {
            return Point::new(x, y);
        }
    }
}

fn player_move<T>(bw: OccupyType, mut f: T) -> T where T: FieldAction {
    loop {
        let p = get_move();
        let (valid, _) = f.is_valid_move(&p, bw);
        if !valid {
            println!(" {} -- is not valid move", p);
            continue;
        }
        let changed = f.move_in_game(&p, bw);
        println!("Player has moved to {}, +{} score", p, changed + 1);
        return f;
    }
}


fn print_field_and_score(computer: OccupyType, f: &dyn FieldAction, win: bool) {
    println!("{}", f.to_string());
    let score_c;
    let score_p;
    let (w, b) = f.get_score_wb();
    if computer == OccupyType::White {
        score_c = w;
        score_p = b;
    } else {
        score_p = w;
        score_c = b;
    }

    println!("Score Player vs Computer -- {}:{}", score_p, score_c);
    if win {
        if score_p == score_c {
            println!("DRAW");
        }
        if score_c > score_p {
            println!("Computer WINS");
        } else {
            println!("Player WINS");
        }
    }
}



fn main() {
    let player = beginning_choice();

    let computer = OccupyType::get_opposite_type(player);
    let mut field = Field::new();
    field.init();

    print_field_and_score(computer, &field, false);

    if computer == OccupyType::White {
        field = computer_move(computer, field);
        print_field_and_score(computer, &field, false);
    }

    let mut player_can_move = true;
    let mut computer_can_move = true;

    while player_can_move || computer_can_move {
        if possible_movement(player, &field) {
            field = player_move(player, field);
            player_can_move = true;
            print_field_and_score(computer, &field, false);
        } else {
            player_can_move = false;
            println!("Player PASS");
        }
        if possible_movement(computer, &field) {
            field = computer_move(computer, field);
            computer_can_move = true;
            print_field_and_score(computer, &field, false);
        } else {
            computer_can_move = false;
            println!("Computer PASS");
        }
    }
    print_field_and_score(computer, &field, true);
}

