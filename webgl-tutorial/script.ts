(function () {
    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 300;
    canvas.height = 300;

    const gl = canvas.getContext('webgl', { stencil: true })!;
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

    interface Attribute {
        loc: number;
        vbo: WebGLBuffer;
        stride: number;
    }

    function createAttribute(name: string, data: number[], stride: number, program: WebGLProgram): Attribute {
        const loc = gl.getAttribLocation(program, name);
        const vbo = createVertexBuffer(data);
        return { loc, vbo, stride };
    }

    function setAttribute(attr: Attribute): void {
        const { loc, vbo, stride } = attr;
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

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.DEPTH_TEST);
        gl.enable(gl.STENCIL_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.enable(gl.BLEND);

        const prog = createProgram(vs, fs);

        // prettier-ignore
        const positions = [
            -1.0, -1.0, 0.0,
             1.0, -1.0, 0.0,
            -1.0,  1.0, 0.0,
             1.0,  1.0, 0.0,
        ];
        const posAttr = createAttribute('position', positions, 3, prog);
        setAttribute(posAttr);

        // prettier-ignore
        const blue = [
            0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
        ];
        const blueAttr = createAttribute('color', blue, 4, prog);

        // prettier-ignore
        const red = [
            1.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
        ];
        const redAttr = createAttribute('color', red, 4, prog);

        // prettier-ignore
        const green = [
            0.0, 1.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
        ];
        const greenAttr = createAttribute('color', green, 4, prog);

        // prettier-ignore
        const indices = [
            0, 1, 2,
            3, 2, 1,
        ];
        const ibo = createIndexBuffer(indices);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);

        const uniforms = ['mvpMat'].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

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

        function update(): void {
            clear();

            m.identity(mouseMat);
            q.toMatIV(qCamera, mouseMat);

            m.lookAt(/* eye position */ [0, 0, 10], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
            m.multiply(vMat, mouseMat, vMat);
            m.multiply(pMat, vMat, vpMat);

            function renderRect(pos: Vec3, attr: Attribute): void {
                setAttribute(attr);
                m.identity(mMat);
                m.translate(mMat, pos, mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms['mvpMat'], /*transpose*/ false, mvpMat);
                gl.drawElements(gl.TRIANGLES, indices.length, gl.UNSIGNED_SHORT, 0);
            }

            gl.stencilFunc(gl.ALWAYS, 1, ~0);
            gl.stencilOp(gl.KEEP, gl.REPLACE, gl.REPLACE);
            renderRect([-0.25, 0.25, -0.5], redAttr);

            gl.stencilFunc(gl.ALWAYS, 0, ~0);
            gl.stencilOp(gl.KEEP, gl.INCR, gl.INCR);
            renderRect([0, 0, 0], blueAttr);

            gl.stencilFunc(gl.EQUAL, 2, ~0);
            gl.stencilOp(gl.KEEP, gl.KEEP, gl.KEEP);
            renderRect([0.25, -0.25, 0.5], greenAttr);

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
