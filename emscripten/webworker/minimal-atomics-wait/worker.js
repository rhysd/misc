let buffer = null;

onmessage = e => {
    const d = e.data;
    if ('buf' in d) {
        buffer = d.buf;
        console.log('worker: buffer was set:', buffer);
    } else {
        Atomics.store(buffer, 0, 0);
        console.log('before wait:', buffer[0], Date.now());
        console.log('Atomics.wait:', Atomics.wait(buffer, 0, 0, 10000));
        console.log('after wait:', buffer[0], Date.now());
        postMessage(`${d.input} ${d.input}`);
    }
    console.log('worker: received:', e);
};
