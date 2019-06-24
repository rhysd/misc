onmessage = e => {
    console.log('start!', e.data);

    const canvas = e.data.canvas;
    const array = e.data.array;
    const ctx = canvas.getContext('2d', { alpha: false });

    let percent = 10;
    while (true) {
        // Wait for animation frame (1 is set to buffer)
        Atomics.store(array, 0, 0);
        Atomics.wait(array, 0, 0);

        // Animation frame
        console.log('render:', percent);
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = 'green';
        const w = Math.floor((canvas.width * percent) / 100);
        const h = Math.floor((canvas.height * percent) / 100);
        ctx.fillRect(0, 0, w, h); // This rendering never happen

        percent = (percent + 10) % 100;
    }
};
