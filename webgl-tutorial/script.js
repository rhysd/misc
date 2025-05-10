'use strict';

(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 300;
    canvas.height = 300;

    const gl = canvas.getContext('webgl');
    const m = new matIV();

    function clear() {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
        gl.clearDepth(1.0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    }

    async function loadShader(path) {
        const res = await fetch(path);
        if (!res.ok) {
            throw new Error(`Fetching ${path} failed with status ${response.status}: ${response.statusText}`);
        }
        const src = await res.text();

        let shader;
        if (path.endsWith('.vert')) {
            shader = gl.createShader(gl.VERTEX_SHADER);
        } else if (path.endsWith('.frag')) {
            shader = gl.createShader(gl.FRAGMENT_SHADER);
        } else {
            throw new Error(`Unknown file extension for shader: ${path}`);
        }

        gl.shaderSource(shader, src);
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

    function createVertexBuffer(data) {
        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ARRAY_BUFFER, null);
        return vbo;
    }

    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        clear();

        const prog = createProgram(vs, fs);

        {
            const loc = gl.getAttribLocation(prog, 'position');
            const stride = 3; // 3 elements (x, y, z)
            // prettier-ignore
            const vertexPos = [
            //     x,   y,   z,
                 0.0, 2.0, 0.0,
                 2.0, 0.0, 0.0,
                -2.0, 0.0, 0.0,
            ];
            // Bind 'position' attribute
            const vbo = createVertexBuffer(vertexPos);
            gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
            gl.enableVertexAttribArray(loc);
            gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
        }

        {
            const loc = gl.getAttribLocation(prog, 'color');
            const stride = 4; // (r, g, b, a)
            // prettier-ignore
            const vertexColors = [
            //    r,   g,   b,   a,
                1.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
            ];
            const vbo = createVertexBuffer(vertexColors);
            gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
            gl.enableVertexAttribArray(loc);
            gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
        }

        const mMat = m.identity(m.create());
        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const mvpMat = m.identity(m.create());

        m.lookAt(/* eye position */ [0, 1, 3], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 90,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat
        );

        // mvp = p * v * m
        m.multiply(pMat, vMat, mvpMat);
        m.multiply(mvpMat, mMat, mvpMat);

        // Define uniform
        const uniMvpLoc = gl.getUniformLocation(prog, 'mvpMat');
        gl.uniformMatrix4fv(uniMvpLoc, false, mvpMat);

        gl.drawArrays(gl.TRIANGLES, /* start from 0th vertex */ 0, /* number of vertice */ 3);
        gl.flush(); // Actual re-rendering happens here
    }

    main().catch(alert);
})();
