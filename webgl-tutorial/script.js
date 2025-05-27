'use strict';

(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 600;
    canvas.height = 400;

    const modeLines = document.getElementById('mode-lines');
    const modeLineStrip = document.getElementById('mode-line-strip');
    const modeLineLoop = document.getElementById('mode-line-loop');
    const pointSizePercent = document.getElementById('pointer-size');

    const gl = canvas.getContext('webgl');
    const m = new matIV();
    const q = new qtnIV();

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

    function createAttribute(name, data, stride, program) {
        const loc = gl.getAttribLocation(program, name);
        const vbo = createVertexBuffer(data);
        return { loc, vbo, stride };
    }

    function setAttribute(attr) {
        const { loc, vbo, stride } = attr;
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.enableVertexAttribArray(loc);
        gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
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

    function sphere(row, col, radius) {
        const positions = [];
        const normals = [];
        const colors = [];
        const indices = [];

        for (let i = 0; i <= row; i++) {
            const rad = (Math.PI / row) * i;
            const ry = Math.cos(rad);
            const rr = Math.sin(rad);
            for (let j = 0; j <= col; j++) {
                const rad = ((Math.PI * 2) / col) * j;

                const x = rr * radius * Math.cos(rad);
                const y = ry * radius;
                const z = rr * radius * Math.sin(rad);
                positions.push(x, y, z);

                const rx = rr * Math.cos(rad);
                const rz = rr * Math.sin(rad);
                normals.push(rx, ry, rz);

                colors.push(...hsva((360 / row) * i, 1, 1, 1));
            }
        }

        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + 1, r + col + 2);
                indices.push(r, r + col + 2, r + col + 1);
            }
        }

        return [positions, normals, colors, indices];
    }

    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.enable(gl.BLEND);

        const prog = createProgram(vs, fs);

        const [spherePositions, , sphereColors] = sphere(16, 16, 2);
        const spherePosAttr = createAttribute('position', spherePositions, 3, prog);
        const sphereColorAttr = createAttribute('color', sphereColors, 4, prog);

        // prettier-ignore
        const linePositions = [
            -1.0, -1.0, 0.0,
             1.0, -1.0, 0.0,
            -1.0,  1.0, 0.0,
             1.0,  1.0, 0.0,
        ];
        const linePosAttr = createAttribute('position', linePositions, 3, prog);

        // prettier-ignore
        const lineColors = [
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
        ];
        const lineColorAttr = createAttribute('color', lineColors, 4, prog);

        const uniforms = ['mvpMat', 'pointSize'].reduce((acc, name) => {
            acc[name] = gl.getUniformLocation(prog, name);
            return acc;
        }, {});

        const pMat = m.identity(m.create());
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );

        const vMat = m.create();
        const vpMat = m.create();
        const mMat = m.create();
        const mvpMat = m.create();
        const qCamera = q.identity(q.create());
        const mouseMat = m.create();
        const [pointSizeMin, pointSizeMax] = gl.getParameter(gl.ALIASED_POINT_SIZE_RANGE);

        canvas.addEventListener(
            'mousemove',
            event => {
                const w = canvas.width;
                const h = canvas.height;
                const x = event.clientX - canvas.offsetLeft - w / 2;
                const y = event.clientY - canvas.offsetTop - h / 2;
                const len = Math.sqrt(x * x + y * y);

                // Normalize position
                const normX = x / len;
                const normY = y / len;

                // Use distance from the center of canvas to calculate the angle
                const diag = Math.sqrt(w * w + h * h);
                const rad = 2 * Math.PI * (len / diag);

                // Calculate quaternion to rotate the model
                q.rotate(rad, [normY, normX, 0], qCamera);
            },
            { passive: true },
        );

        let count = 0;
        function update() {
            clear();

            count++;
            const rad = ((count % 720) * Math.PI) / 360;

            m.identity(mouseMat);
            q.toMatIV(qCamera, mouseMat);

            m.lookAt(/* eye position */ [0, 5, 10], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
            m.multiply(vMat, mouseMat, vMat);
            m.multiply(pMat, vMat, vpMat);

            const pointSize = Math.max(pointSizeMin, Math.min(pointSizeMax, parseFloat(pointSizePercent.value)));
            gl.uniform1f(uniforms.pointSize, pointSize);

            // Render points

            setAttribute(spherePosAttr);
            setAttribute(sphereColorAttr);

            m.identity(mMat);
            m.rotate(mMat, rad, [0, 1, 0], mMat);
            m.multiply(vpMat, mMat, mvpMat);
            gl.uniformMatrix4fv(uniforms.mvpMat, /*transpose*/ false, mvpMat);
            gl.drawArrays(gl.POINTS, 0, spherePositions.length / 3);

            // Render lines

            setAttribute(linePosAttr);
            setAttribute(lineColorAttr);

            m.identity(mMat);
            m.rotate(mMat, Math.PI / 2, [1, 0, 0], mMat);
            m.scale(mMat, [3, 3, 1], mMat);
            m.multiply(vpMat, mMat, mvpMat);
            gl.uniformMatrix4fv(uniforms.mvpMat, /*transpose*/ false, mvpMat);

            let mode;
            if (modeLines.checked) {
                mode = gl.LINES;
            } else if (modeLineStrip.checked) {
                mode = gl.LINE_STRIP;
            } else if (modeLineLoop.checked) {
                mode = gl.LINE_LOOP;
            }

            gl.drawArrays(mode, 0, linePositions.length / 3);

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
