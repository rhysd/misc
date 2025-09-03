(function () {
    type Pos = [number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 600;
    canvas.height = 400;

    const checkbox = document.getElementById('checkbox')! as HTMLInputElement;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();
    const q = new qtnIV();

    function clear(): void {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
        gl.clearDepth(1.0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
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
        gl.bindTexture(gl.TEXTURE_2D, null);

        return tex;
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.enable(gl.BLEND);

        const prog = createProgram(vs, fs);

        // prettier-ignore
        const positions = [
            -1.0,  1.0, 0.0,
             1.0,  1.0, 0.0,
            -1.0, -1.0, 0.0,
             1.0, -1.0, 0.0,
        ];
        const posAttr = createAttribute('position', positions, 3, prog);
        setAttribute(posAttr);

        // prettier-ignore
        const textureCoords = [
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0
        ];
        const textureCoordAttr = createAttribute('textureCoord', textureCoords, 2, prog);
        setAttribute(textureCoordAttr);

        // prettier-ignore
        const indices = [
            0, 1, 2,
            3, 2, 1,
        ];
        const ibo = createIndexBuffer(indices);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);

        // Load texture data and activate textures
        const texBall = await loadTexture2D('../assets/ball.png');
        const texFloor = await loadTexture2D('../assets/floor.png');

        gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE);

        const uniforms = ['mvpMat', 'texture'].reduce(
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
        const vInvMat = m.create();
        const vpMat = m.create();
        const mMat = m.create();
        const mvpMat = m.create();
        const cameraPosInit: Pos = [0, 5, 10];
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

            q.toMatIV(qCamera, mouseMat);

            m.lookAt(/* eye position */ cameraPosInit, /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
            m.multiply(vMat, mouseMat, vMat);
            m.multiply(pMat, vMat, vpMat);

            // Render floor texture

            m.identity(mMat);
            m.rotate(mMat, Math.PI / 2, [1, 0, 0], mMat);
            m.scale(mMat, [3, 3, 1], mMat);
            m.multiply(vpMat, mMat, mvpMat);
            gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
            gl.activeTexture(gl.TEXTURE1);
            gl.bindTexture(gl.TEXTURE_2D, texFloor);
            gl.uniform1i(uniforms.texture, 1);
            gl.drawElements(gl.TRIANGLES, indices.length, /* type of index */ gl.UNSIGNED_SHORT, /* start offset */ 0);

            // Render billboard texture

            m.identity(mMat);
            m.translate(mMat, [0, 1, 0], mMat);

            if (checkbox.checked) {
                m.lookAt([0, 0, 0], cameraPosInit, [0, 1, 0], vInvMat);
                m.multiply(vInvMat, mouseMat, vInvMat);
                m.inverse(vInvMat, vInvMat);
                m.multiply(mMat, vInvMat, mMat);
            }

            m.multiply(vpMat, mMat, mvpMat);
            gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
            gl.activeTexture(gl.TEXTURE0);
            gl.bindTexture(gl.TEXTURE_2D, texBall);
            gl.uniform1i(uniforms.texture, 0);
            gl.drawElements(gl.TRIANGLES, indices.length, /* type of index */ gl.UNSIGNED_SHORT, /* start offset */ 0);

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch((err: Error) => alert(err.stack ?? err.message ?? String(err)));
})();
