import * as wasm from "wasm-rusty-snake";
import {memory} from "wasm-rusty-snake/rusty_snake_bg.wasm";

const width = 30;
const height = 30;

const blockSize = 10;

const colorMap = {
    [wasm.CellType.Empty]: 'rgb(255, 255, 255)',
    [wasm.CellType.Snake]: 'rgb(0,178,0)',
    [wasm.CellType.Food]: 'rgb(190,0,0)',
    [wasm.CellType.Wall]: 'rgb(0,0,0)',
};

function draw(field, canvas) {
    const message = field.cells();
    let bufferSize = width * height;
    const cells = new Uint8Array(memory.buffer, message, bufferSize);

    const ctx = canvas.getContext('2d');

    for (let i = 0; i < cells.length; i++) {
        ctx.fillStyle = colorMap[cells[i]] || 'rgb(225,219,15)';
        const y = Math.floor(i / width);
        const x = i - y * width;
        ctx.fillRect(x * blockSize, y * blockSize, blockSize, blockSize);
    }
}

const canvas = document.getElementById("canvas");
const field = wasm.Field.new(width, height);

const keyDirMapping = {
    "KeyW": wasm.Direction.Up,
    "KeyA": wasm.Direction.Left,
    "KeyD": wasm.Direction.Right,
    "KeyS": wasm.Direction.Down,
};

window.addEventListener("keypress", (e) => {
    console.log(e.code);
    const direction = keyDirMapping[e.code];
    if (direction) {
        field.change_dir(direction);
    }
});

draw(field, canvas);

function p() {
    field.tick();
    draw(field, canvas);
}

setInterval(p, 100);