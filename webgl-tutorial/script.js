'use strict';

(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 500;
    canvas.height = 300;

    const gl = canvas.getContext('webgl');
    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    function createShader(id) {
        const elem = document.getElementById(id);
        if (!elem) {
            throw new Error('<canvas> element is not found');
        }

        let shader;
        switch (elem.type) {
            case 'x-shader/x-vertex':
                shader = gl.createShader(gl.VERTEX_SHADER);
                break;
            case 'x-shader/x-fragment':
                shader = gl.createShader(gl.FRAGMENT_SHADER);
                break;
            default:
                throw new Error(`Unexpected element type: ${elem.type}`);
        }

        gl.shaderSource(shader, elem.text);
        gl.compileShader(shader);

        if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            return shader;
        } else {
            throw new Error(`Shader compilation failed: ${gl.getShaderInfoLog(shader)}`);
        }
    }

    function createProgram(vs, fs) {
        const program = gl.createProgram();

        gl.attachShader(program, vs);
        gl.attachShader(program, fs);

        gl.linkProgram(program);

        if (gl.getProgramParameter(program, gl.LINK_STATUS)) {
            gl.useProgram(program);
            return program;
        } else {
            throw new Error(`Could not craete program: ${gl.getProgramInfoLog(program)}`);
        }
    }

    function main() {
        const vs = createShader('vs');
        const fs = createShader('fs');
        const prog = createProgram(vs, fs);
        console.log(prog);
    }

    try {
        main();
    } catch (err) {
        alert(err);
    }
})();
