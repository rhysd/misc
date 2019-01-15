import { Universe } from 'wasm-tutorial';
import { memory } from 'wasm-tutorial/wasm_tutorial_bg';

const CELL_SIZE = 5;

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
const screen = document.getElementById('screen');
screen.height = (CELL_SIZE + 1) * height + 1;
screen.width = (CELL_SIZE + 1) * width + 1;
const ctx = screen.getContext('2d');
let genCount = 0;
const counter = document.getElementById('gen-count');

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = '#cccccc';

    // Vertical lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines
    for (let i = 0; i <= height; i++) {
        ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function drawCells() {
    const ptr = universe.cells();
    // Devide by 8 since 1 bit represents 1 cell
    const cells = new Uint8Array(memory.buffer, ptr, (width * height) / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = row * width + col;
            const byteIdx = Math.floor(idx / 8);
            const mask = 1 << idx % 8;
            const isDead = (cells[byteIdx] & mask) === mask;
            ctx.fillStyle = isDead ? '#000000' : '#ffffff';
            ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }

    ctx.stroke();
}

function loop() {
    universe.tick();
    drawCells();
    genCount++;
    counter.textContent = genCount;

    requestAnimationFrame(loop);
}

// Start
drawGrid();
requestAnimationFrame(loop);
