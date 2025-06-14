(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 300;
    canvas.height = 300;
    const lightScaleInput = document.getElementById('light-scale')! as HTMLInputElement;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();
    const q = new qtnIV();

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

    function torus(row: number, col: number, innerRadius: number, outerRadius: number, color: Color): ObjectData {
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

        return { positions, normals, colors, indices };
    }

    function rect(size: number, color: Color): ObjectData {
        // prettier-ignore
        const positions = [
            -size, 0.0, -size,
            size,  0.0, -size,
            -size, 0.0,  size,
            size,  0.0,  size,
        ];
        // prettier-ignore
        const normals = [
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
        ];
        const colors = [...color, ...color, ...color, ...color];
        // prettier-ignore
        const indices = [
            0, 1, 2,
            3, 2, 1,
        ];
        return { positions, normals, colors, indices };
    }

    function loadImage(src: string): Promise<HTMLImageElement> {
        const img = new Image();

        return new Promise((resolve, reject) => {
            img.onload = () => {
                resolve(img);
            };
            img.onerror = () => {
                reject(new Error(`Could not load image ${src}`));
            };
            img.src = src;
        });
    }

    async function loadTexture2D(src: string): Promise<WebGLTexture> {
        const img = await loadImage(src);
        const tex = gl.createTexture();

        gl.bindTexture(gl.TEXTURE_2D, tex);
        gl.texImage2D(
            /* target */ gl.TEXTURE_2D,
            /* level of mipmap */ 0,
            /* color components in texture */ gl.RGBA,
            /* format of the texel data*/ gl.RGBA,
            /* 1 byte per element of RGBA */ gl.UNSIGNED_BYTE,
            img,
        );
        gl.generateMipmap(gl.TEXTURE_2D);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.bindTexture(gl.TEXTURE_2D, null);

        return tex;
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const prog = createProgram(vs, fs);

        const texture = await loadTexture2D('assets/ferris-bg.png');
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);

        const torusObject = createObject(prog, torus(64, 64, 1, 2, [1, 1, 1, 1]));
        const rectObject = createObject(prog, rect(1, [1, 1, 1, 1]));

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const tvMat = m.create();
        const tpMat = m.create(); // Projection matrix looking from the light position

        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 150,
            pMat,
        );
        m.perspective(90, 1.0, 0.1, 150, tpMat);

        const cameraPos: Vec3 = [0, 0, 70];
        const cameraUp: Vec3 = [0, 1, 0];
        const qCamera = q.identity(q.create());

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

        const uniforms = ['mMat', 'mvpMat', 'invMat', 'tMat', 'texture'].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

        gl.uniform1i(uniforms.texture, 0);

        const mMat = m.create();
        const mvpMat = m.create();
        const invMat = m.create();
        const lightUpDirection: Vec3 = [0.577, 0.577, -0.577];

        let count = 0;
        function update() {
            clear();

            count++;

            const lightScale = parseFloat(lightScaleInput.value);
            const lightPosition: Vec3 = [-lightScale, lightScale, lightScale];
            m.lookAt(lightPosition, [0, 0, 0], lightUpDirection, tvMat);
            gl.uniform3fv(uniforms.lightPosition, lightPosition);

            q.toVecIII([0, 0, 70], qCamera, cameraPos);
            q.toVecIII([0, 1, 0], qCamera, cameraUp);
            m.lookAt(cameraPos, /* Camera center */ [0, 0, 0], cameraUp, vMat);
            m.multiply(pMat, vMat, vpMat);

            // prettier-ignore
            const tMat = new Float32Array([
                0.5,  0.0, 0.0, 0.0,
                0.0, -0.5, 0.0, 0.0,
                0.0,  0.0, 1.0, 0.0,
                0.5,  0.5, 0.0, 1.0,
            ]);
            m.multiply(tMat, tpMat, tMat);
            m.multiply(tMat, tvMat, tMat);
            gl.uniformMatrix4fv(uniforms.tMat, /* transpose */ false, tMat);

            // Render torus object
            bindBuffers(torusObject);
            for (let i = 0; i < 10; i++) {
                const pos: Vec3 = [((i % 5) - 2) * 7, Math.floor(i / 5) * 7 - 5, ((i % 5) - 2) * 5];
                const rad = (((count + i * 36) % 360) * Math.PI) / 180;

                m.identity(mMat);
                m.translate(mMat, pos, mMat);
                m.rotate(mMat, rad, /* axis */ [1, 1, 0], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);

                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);

                gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, gl.UNSIGNED_SHORT, 0);
            }

            // Render background rects
            bindBuffers(rectObject);

            function drawBackground(): void {
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);
                gl.drawElements(gl.TRIANGLES, rectObject.lenIndices, gl.UNSIGNED_SHORT, 0);
            }

            m.identity(mMat);
            m.translate(mMat, [0.0, -10.0, 0.0], mMat);
            m.scale(mMat, [20.0, 0.0, 20.0], mMat);
            drawBackground();

            m.identity(mMat);
            m.translate(mMat, [0.0, 10.0, -20.0], mMat);
            m.rotate(mMat, Math.PI / 2, [1, 0, 0], mMat);
            m.scale(mMat, [20.0, 0.0, 20.0], mMat);
            drawBackground();

            m.identity(mMat);
            m.translate(mMat, [20.0, 10.0, 0.0], mMat);
            m.rotate(mMat, Math.PI / 2, [0, 0, 1], mMat);
            m.scale(mMat, [20.0, 0.0, 20.0], mMat);
            drawBackground();

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
