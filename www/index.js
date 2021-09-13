import * as wasm from "rustversi";

const BOARD_SIZE = 8;
const WHITE = wasm.js_get_white_sym();
const BLACK = wasm.js_get_black_sym();

//---------------------------------------------------------------------
class ViewPort {

    constructor() {
        this._SIZE = 38;
        this._SIZE2 = this._SIZE / 2;
        this._PI2 = 2 * Math.PI;
        this._setSize();
    }

    _setSize() {
        for (let x = 1; x < 9; x++) {
            for (let y = 1; y < 9; y++) {
                const elem = document.getElementById("pos" + x + "" + y);
                elem.width = this._SIZE;
                elem.height = this._SIZE;
            }
        }
        let el = document.getElementById("white");
        el.width = this._SIZE;
        el.height = this._SIZE;
        el = document.getElementById("black");
        el.width = this._SIZE;
        el.height = this._SIZE;
    }

    _drawFigure(name, sym) {
        const canvas = document.querySelector(name);
        const context = canvas.getContext("2d");

        context.fillStyle = "#fff";
        context.fillRect(0, 0, this._SIZE, this._SIZE);

        if (sym === BLACK) {
            context.fillStyle = "#f00";
            context.fillRect(0, 0, this._SIZE, this._SIZE);
        } else if (sym === WHITE) {
            context.beginPath();
            context.arc(this._SIZE2, this._SIZE2, this._SIZE2, 0, this._PI2, true);
            context.fillStyle = "#00f";
            context.fill();
        }
    }

    drawPoint(x, y, sym) {
        this._drawFigure("#pos" + x + "" + y, sym);
    }

    _div(val, by) {
        return (val - val % by) / by;
    }

    drawField(s) {
        for (let i = 0; i < s.length; i++) {
            this.drawPoint((i % BOARD_SIZE + 1), BOARD_SIZE - this._div(i, BOARD_SIZE), s.charAt(i));
        }
    }

    drawSelect() {
        this._drawFigure("#white", WHITE);
        this._drawFigure("#black", BLACK);
    }

    printMessage(s) {
        console.log(s);
        document.getElementById('log').innerHTML = s;
    }

    showSelect() {
        document.getElementById("select").style.display = "inline";
    }

    hideSelect() {
        document.getElementById("select").style.display = "none";
    }
}
//---------------------------------------------------------------------

class Game {

    constructor(vp) {
        this._viewPort = vp;
        this._player = "";
        this._computer = "";
        this._fieldStr = wasm.js_field_init();

        this._viewPort.drawSelect();
        this._viewPort.printMessage("");
        this._viewPort.drawField(this._fieldStr);
    }

    _parseScore(s) {
        const list = s.trim().split(":");
        return [parseInt(list[0]), parseInt(list[1])];
    }

    start(color) {
        this._fieldStr = wasm.js_field_init();
        this._viewPort.printMessage("");
        this._player = color;
        this._computer = wasm.js_get_opposite_sym(this._player);
        if (this._computer === WHITE) {
            this._fieldStr = wasm.js_computer_move(this._computer, this._fieldStr);
        }
        this._viewPort.drawField(this._fieldStr);
        console.log(wasm.js_print_field_and_score(this._computer, this._fieldStr, false));
    }

    _playerMove(x, y) {
        if (wasm.js_possible_movement(this._player, this._fieldStr)) {
            const valid = wasm.js_check_point(x, y) &&
                wasm.js_is_valid_move(x, y, this._player, this._fieldStr);
            if (!valid) {
                this._viewPort.printMessage("this is not valid move: <br> (" + x + ", " + y + ")");
                return false;
            }
            this._fieldStr = wasm.js_player_move(x, y, this._player, this._fieldStr);
            console.log("Player has moved to " + x + "," + y);
            this._viewPort.drawField(this._fieldStr);
            console.log(wasm.js_print_field_and_score(this._computer, this._fieldStr, false));
        } else {
            this._viewPort.printMessage("Player PASS");
        }
        return true;
    }

    _computerMove(x, y) {
        let playerСanMove = true;
        let computerCanMove = true;

        if (wasm.js_possible_movement(this._computer, this._fieldStr)) {
            this._fieldStr = wasm.js_computer_move(this._computer, this._fieldStr);
            this._viewPort.drawField(this._fieldStr);
            console.log(wasm.js_print_field_and_score(this._computer, this._fieldStr, false));
            playerСanMove = wasm.js_possible_movement(this._player, this._fieldStr);
            while (!playerСanMove) {
                if (wasm.js_possible_movement(this._computer, this._fieldStr)) {
                    this._fieldStr = wasm.js_computer_move(this._computer, this._fieldStr);
                    this._viewPort.drawField(this._fieldStr);
                    console.log(wasm.js_print_field_and_score(this._computer, this._fieldStr, false));
                } else {
                    computerCanMove = false;
                }
                playerСanMove = wasm.js_possible_movement(this._player, this._fieldStr);
            }
        } else {
            computerCanMove = false;
            playerСanMove = wasm.js_possible_movement(this._player, this._fieldStr);
            this._viewPort.printMessage("Computer PASS");
        }
        return [playerСanMove, computerCanMove];
    }

    moveTo(x, y) {
        if (this._player === "") {
            this._viewPort.printMessage("Select color");
            return;
        }
        this._viewPort.printMessage("");

        if (!this._playerMove(x, y)) {
            return;
        }
        const [playerСanMove, computerCanMove] = this._computerMove(x, y);

        if (!playerСanMove && !computerCanMove) {
            console.log(wasm.js_print_field_and_score(this._computer, this._fieldStr, true));

            const score = wasm.js_get_score_pc(this._computer, this._fieldStr);
            const res = this._parseScore(score);
            let result = "DRAW";
            if (res[0] > res[1]) {
                result = "Player WINS";
            } else if (res[0] < res[1]) {
                result = "Computer WINS";
            }
            this._viewPort.printMessage(score + " <br> " + result);
            this._viewPort.showSelect();
        }
    }
}
//--------------------------------------------------------

const viewPort = new ViewPort();
const game = new Game(viewPort);

const fieldBody = document.querySelector('#board tbody');
fieldBody.addEventListener('click', function (e) {
    const cell = e.target.closest('td');
    if (!cell) { return; }
    const row = cell.parentElement;
    const x = cell.cellIndex + 1;
    const y = BOARD_SIZE - row.rowIndex;
    game.moveTo(x, y);
});

const selectBody = document.querySelector('#select tbody');
selectBody.addEventListener('click', function (e) {
    const cell = e.target.closest('td');
    if (!cell) { return; }
    const x = cell.cellIndex;
    game.start(x == 0 ? WHITE : BLACK);
    viewPort.hideSelect();
});



//EoF