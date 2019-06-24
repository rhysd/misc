const canvas = (function() {
    const c = document.getElementById('canvas');
    const r = c.getBoundingClientRect();
    const d = window.devicePixelRatio;
    c.width = r.width * d;
    c.height = r.height * d;
    return c;
})();
const offscreen = canvas.transferControlToOffscreen();
const buffer = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 1);
const array = new Int32Array(buffer);
const worker = new Worker('./renderer.js');
worker.postMessage({ canvas: offscreen, array }, [offscreen]);

function raf() {
    // Notify animation frame to worker thread
    Atomics.store(array, 0, 1);
    Atomics.notify(array, 0, 1);

    window.requestAnimationFrame(raf);
}
window.requestAnimationFrame(raf);
