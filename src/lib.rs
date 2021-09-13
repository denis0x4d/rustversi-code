use crate::point::{Point};
use crate::occupytype::OccupyType;
use crate::field::{Field, FieldAction};
use crate::game::{computer_move, possible_movement};
use wasm_bindgen::prelude::*;

pub mod point;
pub mod occupytype;
pub mod field;
pub mod game;


#[wasm_bindgen]
pub fn js_print_field_and_score(comp: &str, field_str: &str, win: bool) ->String {

    let computer = OccupyType::from_string(&comp.to_string());
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());

    let mut ret_value=format!("{}\n", f.to_string());
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
    ret_value.push_str(format!("Score Player vs Computer -- {}:{}\n", score_p, score_c).as_str());
    if win {
        if score_p == score_c {
            ret_value.push_str(format!("DRAW").as_str());
        }
        if score_c > score_p {
            ret_value.push_str(format!("Computer WINS").as_str());
        } else {
            ret_value.push_str(format!("Player WINS").as_str());
        }
    }
    return ret_value;
}

#[wasm_bindgen]
pub fn js_get_black_sym() -> String {
    return OccupyType::Black.to_string();
}

#[wasm_bindgen]
pub fn js_get_white_sym() -> String {
    return OccupyType::White.to_string();
}

#[wasm_bindgen]
pub fn js_get_opposite_sym(bw: &str) -> String {
    return OccupyType::get_opposite_type(OccupyType::from_string(&bw.to_string())).to_string();
}

#[wasm_bindgen]
pub fn js_field_init() -> String {
    let mut f = Field::new();
    f.init();
    return f.serialize();
}

#[wasm_bindgen]
pub fn js_computer_move(computer: &str, field_str: &str) -> String {
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());
    f = computer_move(OccupyType::from_string(&computer.to_string()), f);
    return f.serialize();
}

#[wasm_bindgen]
pub fn js_possible_movement(jbw: &str, field_str: &str) -> bool {
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());
    let bw = OccupyType::from_string(&jbw.to_string());
    return possible_movement(bw, &f);
}

#[wasm_bindgen]
pub fn js_check_point(x: i8, y: i8) -> bool {
    return Point::check_point(x as i8, y as i8);
}

#[wasm_bindgen]
pub fn js_is_valid_move(x: i8, y: i8, jbw: &str, field_str: &str) -> bool {
    if !js_check_point(x, y ){
        return false;
    }
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());
    let bw = OccupyType::from_string(&jbw.to_string());
    let p = Point::new(x as u8, y as u8);
    let (valid, _) = f.is_valid_move(&p, bw);
    return valid;
}

#[wasm_bindgen]
pub fn js_player_move(x: i8, y: i8, jbw: &str, field_str: &str)->String{
    if !js_check_point(x, y ){
        return field_str.to_string();
    }
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());
    let bw = OccupyType::from_string(&jbw.to_string());
    let p = Point::new(x as u8, y as u8);
    f.move_in_game(&p, bw);
    return f.serialize();
}

//score player:computer
#[wasm_bindgen]
pub fn js_get_score_pc(comp: &str,field_str: &str)->String{
    let mut f = Field::new();
    f.deserialize(&field_str.to_string());
    let (w, b) = f.get_score_wb();
    let computer = OccupyType::from_string(&comp.to_string());
    let score_c;
    let score_p;
    if computer == OccupyType::White {
        score_c = w;
        score_p = b;
    } else {
        score_p = w;
        score_c = b;
    }
    return format!("{}:{}",score_p,score_c);
}

/* 
////////////////////////////////////////////////////////////////////////////////
JS usage example (../node/nodever.js)

const readline = require('readline');
const wasm=require ('./pkg/rustversi.js');

async function getLine(s){
    return new Promise(function(resolve, reject) {
        const rl = readline.createInterface({
          input: process.stdin,
          output: process.stdout
        });
        rl.question(s, (answer) =>{ rl.close(); resolve(answer);});
    });
}


async function getBeginingChoice(){
    while(true){
        let line = await getLine(wasm.js_get_white_sym()+" - 1,"+wasm.js_get_black_sym()+" - 2 ? ");
     
        const parsed = parseInt(line, 10);
        if (!isNaN(parsed) ) { 	     	  
            if(parsed===1)
                return wasm.js_get_white_sym();
            if(parsed===2)
                return wasm.js_get_black_sym();
        }
    }
}


async function getMove(){
    while(true){
        let line = await getLine("Your turn (x y) >");
    
        arr=line.trim().split(" ");
        if (arr.length==2) {
            const parsed_1 = parseInt(arr[0], 10);
            const parsed_2 = parseInt(arr[1], 10);
            if (!isNaN(parsed_1) && !isNaN(parsed_2) )     	  
                return [parsed_1,parsed_2];
         }
    }
}

(async () => {
    console.log("start...\n");
    const player=await getBeginingChoice();
    const computer = wasm.js_get_opposite_sym(player);
    let field_str=wasm.js_field_init();
    console.log(wasm.js_print_field_and_score(computer,field_str,false));
  
    if (computer===wasm.js_get_white_sym()){
        field_str = wasm.js_computer_move(computer, field_str);
        console.log(wasm.js_print_field_and_score(computer,field_str,false));
    } 
    let player_can_move = true;
    let computer_can_move = true;

    while (player_can_move || computer_can_move) {
 
        if (wasm.js_possible_movement(player, field_str)) {
            let valid=false;
            let xy=[0,0];
            while (!valid) {
                xy = await getMove();
                while (!wasm.js_check_point(xy[0] , xy[1] )) {
                    xy = await getMove();
                }
                valid= wasm.js_is_valid_move(xy[0],xy[1],player,field_str);
                if (!valid) {
                    console.log(xy[0]+","+xy[1]+" -- is not valid move");
                }
            }
            field_str=wasm.js_player_move(xy[0],xy[1],player,field_str);
            console.log("Player has moved to "+xy[0]+","+xy[1]);
            player_can_move = true;
            console.log(wasm.js_print_field_and_score(computer,field_str,false));

        } else {
            player_can_move = false;
            console.log("Player PASS");
        }
   
        if (wasm.js_possible_movement(computer, field_str)) {
            field_str = wasm.js_computer_move(computer, field_str);
            computer_can_move = true;
            console.log(wasm.js_print_field_and_score(computer,field_str,false));
        } else {
            computer_can_move = false;
            console.log("Computer PASS");
        }
    }//while (player_can_move || computer_can_move)
    console.log(wasm.js_print_field_and_score(computer,field_str,true));
})();

////////////////////////////////////////////////////////////////////////////////
RUST usage example: see ./bin/rustversi.rs

*/

