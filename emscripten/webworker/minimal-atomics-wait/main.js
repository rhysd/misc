if (!window.Worker) {
    throw new Error('Worker is not supported');
}

const worker = new Worker('worker.js');
const shared = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 8);
const buffer = new Int32Array(shared);

worker.postMessage({ buf: buffer });

const input = document.getElementById('input');
const submit = document.getElementById('submit');
const output = document.getElementById('output');

submit.addEventListener('click', () => {
    const msg = input.value;
    worker.postMessage({ input: msg });
    console.log('submitted:', msg);
    setTimeout(() => {
        Atomics.store(buffer, 0, 1);
        console.log('wrote to buffer!');
    }, 1500);
});

worker.onmessage = e => {
    output.textContent = e.data;
    console.log('received:', e);
};
