(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 300;
    canvas.height = 300;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();

    function clear(): void {
        gl.clearColor(0.5, 0.7, 1.0, 1.0);
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

    interface RenderObject {
        attrs: Attribute[];
        ibo: WebGLBuffer;
        lenIndices: number;
    }

    interface ObjectData {
        positions: number[];
        colors: number[];
        normals: number[];
        indices: number[];
    }

    function createObject(prog: WebGLProgram, data: ObjectData): RenderObject {
        const { positions, colors, normals, indices } = data;
        return {
            attrs: [
                createAttribute('position', positions, 3, prog),
                createAttribute('color', colors, 4, prog),
                createAttribute('normal', normals, 3, prog),
            ],
            ibo: createIndexBuffer(indices),
            lenIndices: indices.length,
        };
    }

    function createIndexBuffer(data: number[]): WebGLBuffer {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }

    function bindBuffers(object: RenderObject): void {
        for (const attr of object.attrs) {
            const { loc, vbo, stride } = attr;
            gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
            gl.enableVertexAttribArray(loc);
            gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
        }
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, object.ibo);
    }

    function hsva(h: number, s: number, v: number, a: number): Color {
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

    function torus(row: number, col: number, innerRadius: number, outerRadius: number): ObjectData {
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

        return { positions, normals, colors, indices };
    }

    function sphere(row: number, col: number, radius: number): ObjectData {
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

        return { positions, normals, colors, indices };
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const prog = createProgram(vs, fs);

        const torusObject = createObject(prog, torus(64, 64, 0.5, 2.5));
        const sphereObject = createObject(prog, sphere(64, 64, 1.5));

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const lightDirection: [number, number, number] = [-0.5, 0.5, 0.5];

        m.lookAt(/* eye position */ [0, 0, 10], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );
        m.multiply(pMat, vMat, vpMat);

        const uniforms = [
            'mvpMat',
            'invMat',
            'lightDirection',
            'isOutline',
            'toonTexture',
            'toonThresholds',
            'toonGradation',
        ].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

        gl.uniform3fv(uniforms.lightDirection, lightDirection);
        gl.uniform1i(uniforms.toonTexture, 0);
        gl.uniform1fv(uniforms.toonThresholds, [0.2, 0.5, 1.0]); // Thresholds of light gradation
        gl.uniform1fv(uniforms.toonGradation, [0.5, 0.7, 1.0]);

        const mMat = m.create();
        const mvpMat = m.create();
        const invMat = m.create();

        let count = 0;
        function update() {
            clear();

            count++;
            const rad = ((count % 720) * Math.PI) / 360;

            function drawElementsWithOutline(lenIndices: number): void {
                // Render the model normally to only the front face. `gl.BACK` means removing the back face.
                gl.cullFace(gl.BACK);
                gl.uniform1i(uniforms.isOutline, 0);
                gl.drawElements(gl.TRIANGLES, lenIndices, gl.UNSIGNED_SHORT, 0);

                // Render the outline to only the back face. `gl.FRONT` means removing the front face.
                gl.cullFace(gl.FRONT);
                gl.uniform1i(uniforms.isOutline, 1);
                gl.drawElements(gl.TRIANGLES, lenIndices, gl.UNSIGNED_SHORT, 0);
            }

            // Render torus object
            {
                bindBuffers(torusObject);

                m.identity(mMat);
                m.rotate(mMat, rad, /* axis */ [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);

                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);

                drawElementsWithOutline(torusObject.lenIndices);
            }

            // Render sphere object
            {
                bindBuffers(sphereObject);

                m.identity(mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);

                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);

                drawElementsWithOutline(sphereObject.lenIndices);
            }

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
