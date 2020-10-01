import * as wasm from "wasm-si2";
import { memory } from "wasm-si2/wasm_si2_bg";

const WIDTH = 84;
const HEIGHT = 48;

canvas.width = WIDTH * 10;
canvas.height = HEIGHT * 10;
const c = canvas.getContext('2d');

window.game = wasm.Game.new();
const context = wasm.Context.new();

onkeydown = ev => {
    switch (ev.key) {
        case "ArrowLeft":
            context.key_left = true;
            break;
        case "ArrowUp":
            context.key_up = true;
            break;
        case "ArrowRight":
            context.key_right = true;
            break;
        case "ArrowDown":
            context.key_down = true;
            break;
        case " ":
            context.key_space = true;
            break;
        case "a":
            context.key_a = true;
            break;
        default:
            break;
    }
};

onkeyup = ev => {
    switch (ev.key) {
        case "ArrowLeft":
            context.key_left = false;
            break;
        case "ArrowUp":
            context.key_up = false;
            break;
        case "ArrowRight":
            context.key_right = false;
            break;
        case "ArrowDown":
            context.key_down = false;
            break;
        case " ":
            context.key_space = false;
            break;
        case "a":
            context.key_a = false;
            break;
        default:
            break;
    }
};

const tick = () => {
    window.game.update(context);
    window.game.do_render();

    c.fillStyle = window.game.inverted ? 'black' : 'white';
    c.fillRect(0, 0, canvas.width, canvas.height);
    const screen = new Uint8Array(memory.buffer, window.game.screen_pointer(), WIDTH * HEIGHT);

    c.fillStyle = window.game.inverted ? 'white' : 'black';
    for (let y = 0; y < HEIGHT; y++) {
        for (let x = 0; x < WIDTH; x++) {
            if (screen[y * WIDTH + x] == 1) {
                c.fillRect(x * 10, y * 10, 10, 10);
            }
        }
    }

    setTimeout(tick, 30);
};

tick();
