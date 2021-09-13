use crate::point::{Point};
use crate::occupytype::OccupyType;
use crate::field::{FieldAction};
use rand::prelude::*;
//use wasm_bindgen::prelude::*;

pub fn predict_player_corner<T: Clone>(computer: OccupyType, mv: &Point, mut f: T) -> bool where T: FieldAction {
    f.move_in_game(&mv, computer);
    let moves = f.get_list_of_moves(OccupyType::get_opposite_type(computer));

    for (p, _) in moves {
        if p.is_corner() {
            return true;
        }
    }
    return false;
}


pub fn computer_get_best_moves<T: Clone>(check_corner: bool, bw: OccupyType, moves: &Vec<(Point, u8)>, f: T) -> Vec<Point> where T: FieldAction {
    let mut corner = false;
    let mut init = false;
    let mut max_point: Vec<Point> = Vec::new();
    let mut max: u8 = 0;
    if moves.len() > 0 {
        for (p, n) in moves {
            if check_corner && predict_player_corner(bw, &p, f.clone()) {
                continue;
            }
            if init {
                init = false;
                max_point = vec![*p];
                max = *n;
                if p.is_corner() {
                    corner = true;
                }
            }
            if p.is_corner() {
                if corner {
                    if *n > max {
                        max_point = vec![*p];
                        max = *n;
                    } else if *n == max {
                        max_point.push(*p);
                    }
                } else {
                    corner = true;
                    max_point = vec![*p];
                    max = *n;
                }
            } else if !corner {
                if *n > max {
                    max_point = vec![*p];
                    max = *n;
                } else if *n == max {
                    max_point.push(*p);
                }
            }
        }
    }
    return max_point;
}

pub fn computer_move<T: Clone>(bw: OccupyType, mut f: T) -> T where T: FieldAction {
    let moves = f.get_list_of_moves(bw);

    let mut max_point = computer_get_best_moves(true, bw, &moves, f.clone());
    if max_point.len() == 0 {
        max_point = computer_get_best_moves(false, bw, &moves, f.clone());
    }
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..max_point.len());
    println!("Options: {}, selected: {}", max_point.len(), idx);
    let changed = f.move_in_game(&max_point[idx], bw);
    println!("Computer has moved to {}, +{} score", max_point[idx], changed + 1);

    return f;
}


pub fn possible_movement(bw: OccupyType, f: &dyn FieldAction) -> bool {
//TODO: add cache for computer moves
    let moves = f.get_list_of_moves(bw);
    return moves.len() > 0;
}


