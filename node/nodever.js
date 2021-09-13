const readline = require('readline');
const wasm=require ('../pkg/rustversi.js');

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


