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

    function hsva(h, s, v, a) {
        if (s > 1 || v > 1 || a > 1) {
            throw new Error(`Invalid HSVA color (${h}, ${s}, ${v}, ${a})`);
        }
        const th = h % 360;
        const i = Math.floor(th / 60);
        const f = th / 60 - i;
        const m = v * (1 - s);
        const n = v * (1 - s * f);
        const k = v * (1 - s * (1 - f));
        const r = [v, n, m, m, k, v][i];
        const g = [k, v, v, n, m, m][i];
        const b = [m, m, k, v, v, n][i];
        return [r, g, b, a];
    }

    function torus(row, col, innerRadius, outerRadius) {
        const positions = [];
        const colors = [];
        const indices = [];

        for (let i = 0; i <= row; i++) {
            const rad = ((Math.PI * 2) / row) * i;
            const rx = Math.cos(rad);
            const ry = Math.sin(rad);
            for (let j = 0; j <= col; j++) {
                const rad = ((Math.PI * 2) / col) * j;
                const x = (rx * innerRadius + outerRadius) * Math.cos(rad);
                const y = ry * innerRadius;
                const z = (rx * innerRadius + outerRadius) * Math.sin(rad);
                positions.push(x, y, z);
                colors.push(...hsva((360 / col) * j, 1, 1, 1));
            }
        }

        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + col + 1, r + 1);
                indices.push(r + col + 1, r + col + 2, r + 1);
            }
        }

        return [positions, colors, indices];
    }

    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const [positions, colors, indices] = torus(32, 32, 1, 2);

        const prog = createProgram(vs, fs);
        setAttribute('position', positions, 3, prog);
        setAttribute('color', colors, 4, prog);

        const ibo = createIndexBuffer(indices);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const mvpMat = m.identity(m.create());

        m.lookAt(/* eye position */ [0, 0, 20], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 45,
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

            m.rotate(m.identity(mMat), rad, /* axis */ [0, 1, 1], mMat);
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
