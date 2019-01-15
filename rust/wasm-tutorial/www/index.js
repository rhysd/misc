import { Universe } from 'wasm-tutorial';
import { memory } from 'wasm-tutorial/wasm_tutorial_bg';

const CELL_SIZE = 5;

const dpr = window.devicePixelRatio || 1;
const screen = document.getElementById('screen');
const rect = screen.getBoundingClientRect();
screen.height = rect.height * dpr;
screen.width = rect.width * dpr;

const universe = Universe.new(rect.width / CELL_SIZE, rect.height / CELL_SIZE);
const width = universe.width();
const height = universe.height();
const ctx = screen.getContext('2d');

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = '#cccccc';

    // Vertical lines
    const y = ((CELL_SIZE + 1) * height + 1) * dpr;
    for (let i = 0; i <= width; i++) {
        const x = (i * (CELL_SIZE + 1) + 1) * dpr;
        ctx.moveTo(x, 0);
        ctx.lineTo(x, y);
    }

    // Horizontal lines
    const x = ((CELL_SIZE + 1) * width + 1) * dpr;
    for (let i = 0; i <= height; i++) {
        const y = (i * (CELL_SIZE + 1) + 1) * dpr;
        ctx.moveTo(0, y);
        ctx.lineTo(x, y);
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
            const x = (col * (CELL_SIZE + 1) + 1) * dpr;
            const y = (row * (CELL_SIZE + 1) + 1) * dpr;
            ctx.fillRect(x, y, CELL_SIZE * dpr, CELL_SIZE * dpr);
        }
    }

    ctx.stroke();
}

function loop() {
    universe.tick();
    drawCells();
    requestAnimationFrame(loop);
}

// Start
drawGrid();
requestAnimationFrame(loop);
