(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 500;
    canvas.height = 300;

    const gl = canvas.getContext('webgl');
    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
})();
