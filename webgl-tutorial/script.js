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

    function setAttribute(name, data, stride, program) {
        const loc = gl.getAttribLocation(program, name);
        const vbo = createVertexBuffer(data);
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.enableVertexAttribArray(loc);
        gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
    }

    function createIndexBuffer(data) {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }

    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        const prog = createProgram(vs, fs);
        setAttribute(
            'position',
            // prettier-ignore
            [
                // x,    y,   z,
                 0.0,  1.0, 0.0,
                 1.0,  0.0, 0.0,
                -1.0,  0.0, 0.0,
                 0.0, -1.0, 0.0,
            ],
            3, // 3 elements (x, y, z)
            prog,
        );
        setAttribute(
            'color',
            // prettier-ignore
            [
              //  r,   g,   b,   a,
                1.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
            ],
            4, // (r, g, b, a)
            prog,
        );

        // prettier-ignore
        const indices = [
            0, 1, 2, // First triangle
            1, 2, 3, // Second triangle
        ];
        const ibo = createIndexBuffer(indices);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const mvpMat = m.identity(m.create());

        m.lookAt(/* eye position */ [0, 0, 3], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 90,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );
        m.multiply(pMat, vMat, vpMat);

        const uniMvpLoc = gl.getUniformLocation(prog, 'mvpMat');
        const mMat = m.create();

        let count = 0;
        function update() {
            clear();

            count++;
            const rad = ((count % 360) * Math.PI) / 180;

            m.rotate(m.identity(mMat), rad, /* axis */ [0, 1, 0], mMat);
            m.multiply(vpMat, mMat, mvpMat);
            gl.uniformMatrix4fv(uniMvpLoc, false, mvpMat);

            // Draw triangles based on the index buffer.
            gl.drawElements(gl.TRIANGLES, indices.length, /* type of index */ gl.UNSIGNED_SHORT, /* start offset */ 0);

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(alert);
})();
