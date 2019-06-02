if (!window.Worker) {
    throw new Error('Worker is not supported');
}

const worker = new Worker('worker.js');
const shared = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 1);
const buffer = new Int32Array(shared);

const input = document.getElementById('input');
const submit = document.getElementById('submit');
const output = document.getElementById('output');

function notPreparedYet() {
    alert('Not prepared yet!');
}
submit.addEventListener('click', notPreparedYet);

worker.onmessage = e => {
    const data = e.data;
    console.log('main: onmessage: data:', data);
    switch (data.type) {
        case 'init':
            submit.removeEventListener('click', notPreparedYet);
            submit.addEventListener('click', () => {
                const i = input.value | 0;
                worker.postMessage({ type: 'input', input: i });
                console.log('main: submitted:', i);

                if (Math.random() < 0.5) {
                    console.log('main: User input will happen after 1500ms');
                    setTimeout(() => {
                        Atomics.store(buffer, 0, 1);
                        console.log('main: notify user input by writing to buffer!');
                    }, 1500);
                } else {
                    console.log('main: User input will not happen. Worker will wait until timeout');
                }
            });
            console.log('main: initialization done');
            break;
        // case 'result':
        //     output.textContent = e.data;
        //     console.log('received:', e);
        //     break;
        default:
            console.error('main: Unexpected data type:', data);
            break;
    }
};

worker.postMessage({ type: 'init', buf: buffer });
console.log('main: Sent init message');
