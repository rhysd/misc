(function () {
    type Color = [number, number, number, number];
    type Pos = [number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 300;
    canvas.height = 300;

    const range = document.getElementById('range')! as HTMLInputElement;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();
    const q = new qtnIV();

    function clear(): void {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
        gl.clearDepth(1.0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT | gl.STENCIL_BUFFER_BIT);
    }

    async function loadShader(path: string): Promise<WebGLShader> {
        const res = await fetch(path);
        if (!res.ok) {
            throw new Error(`Fetching ${path} failed with status ${res.status}: ${res.statusText}`);
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
        if (!shader) {
            throw new Error(`Shader could not be created for ${path}`);
        }

        gl.shaderSource(shader, src);
        gl.compileShader(shader);

        if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            return shader;
        } else {
            throw new Error(`Shader compilation failed: ${gl.getShaderInfoLog(shader)}`);
        }
    }

    function createProgram(vs: WebGLShader, fs: WebGLShader): WebGLProgram {
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

    function createVertexBuffer(data: number[]): WebGLBuffer {
        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ARRAY_BUFFER, null);
        return vbo;
    }

    function setAttribute(name: string, data: number[], stride: number, program: WebGLProgram): void {
        const loc = gl.getAttribLocation(program, name);
        const vbo = createVertexBuffer(data);
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.enableVertexAttribArray(loc);
        gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
    }

    function createIndexBuffer(data: number[]): WebGLBuffer {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }

    function torus(
        row: number,
        col: number,
        innerRadius: number,
        outerRadius: number,
        color: Color,
    ): [number[], number[], number[], number[]] {
        const positions = [];
        const normals = [];
        const colors = [];
        const indices = [];

        for (let i = 0; i <= row; i++) {
            const rad = ((Math.PI * 2) / row) * i;
            const rr = Math.cos(rad);
            const ry = Math.sin(rad);
            for (let j = 0; j <= col; j++) {
                const rad = ((Math.PI * 2) / col) * j;

                const x = (rr * innerRadius + outerRadius) * Math.cos(rad);
                const y = ry * innerRadius;
                const z = (rr * innerRadius + outerRadius) * Math.sin(rad);
                positions.push(x, y, z);

                const rx = rr * Math.cos(rad);
                const rz = rr * Math.sin(rad);
                normals.push(rx, ry, rz);

                colors.push(...color);
            }
        }

        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + col + 1, r + 1);
                indices.push(r + col + 1, r + col + 2, r + 1);
            }
        }

        return [positions, normals, colors, indices];
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const [positions, normals, colors, indices] = torus(32, 32, 0.5, 1.5, [0.5, 0.5, 0.5, 1]);

        const prog = createProgram(vs, fs);
        setAttribute('position', positions, 3, prog);
        setAttribute('normal', normals, 3, prog);
        setAttribute('color', colors, 4, prog);

        const ibo = createIndexBuffer(indices);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);

        const uniforms = ['mvpMat', 'invMat', 'lightDirection', 'eyeDirection', 'ambientColor'].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

        const mvpMat = m.create();
        const mMat = m.create();
        const vMat = m.create();
        const pMat = m.create();
        const vpMat = m.create();
        const invMat = m.create();
        const lightDirection = [-0.5, 0.5, 0.5];
        const qMouse = q.identity(q.create());
        const cameraPosition: Pos = [0, 0, 20];
        const qRotateX = q.create();
        const qRotateY = q.create();
        const qRotateInterp = q.create();
        let time = parseFloat(range.value);

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
                q.rotate(rad, [normY, normX, 0], qMouse);
            },
            { passive: true },
        );

        range.addEventListener(
            'input',
            event => {
                time = parseFloat((event.target! as HTMLInputElement).value);
            },
            { passive: true },
        );

        m.lookAt(/* eye position */ cameraPosition, /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );
        m.multiply(pMat, vMat, vpMat);

        let count = 0;
        function update() {
            clear();

            count++;
            const rad = ((count % 360) * Math.PI) / 180;

            const rotateMat = m.create();
            const mouseMat = m.create();
            q.toMatIV(qMouse, mouseMat);

            q.rotate(rad, [1, 0, 0], qRotateX);
            q.rotate(rad, [0, 1, 0], qRotateY);
            q.slerp(qRotateX, qRotateY, time, qRotateInterp);

            for (const [ambientColor, qRotate] of [
                [[0.0, 0.0, 0.5, 1.0], qRotateX] as const,
                [[0.5, 0.0, 0.0, 1.0], qRotateY] as const,
                [[0.0, 0.5, 0.0, 1.0], qRotateInterp] as const,
            ]) {
                m.multiply(m.identity(mMat), mouseMat, mMat);

                q.toMatIV(qRotate, rotateMat);
                m.multiply(mMat, rotateMat, mMat);

                m.translate(mMat, [0, 0, -5.0], mMat);

                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);

                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);
                gl.uniform3fv(uniforms.lightDirection, lightDirection);
                gl.uniform3fv(uniforms.eyeDirection, cameraPosition);
                gl.uniform4fv(uniforms.ambientColor, ambientColor);

                // Draw triangles based on the index buffer.
                gl.drawElements(
                    gl.TRIANGLES,
                    indices.length,
                    /* type of index */ gl.UNSIGNED_SHORT,
                    /* start offset */ 0,
                );
            }

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
