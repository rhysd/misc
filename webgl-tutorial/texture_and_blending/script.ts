(function () {
    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 300;
    canvas.height = 300;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();

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
        const colors = [
            1.0, 0.0, 0.0, 0.8,
            0.0, 1.0, 0.0, 0.8,
            0.0, 0.0, 1.0, 0.8,
            1.0, 1.0, 0.0, 0.8,
        ];
        const colorAttr = createAttribute('color', colors, 4, prog);
        setAttribute(colorAttr);

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
        const texture0 = await loadTexture2D('../assets/ferris.png');
        const texture1 = await loadTexture2D('../assets/rust-logo.png');

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());

        m.lookAt(/* eye position */ [0, 0, 10], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );
        m.multiply(pMat, vMat, vpMat);

        const uniforms = ['mvpMat', 'texture0', 'texture1', 'useTexture'].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

        // Bind textures to the texture units
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture0);
        gl.uniform1i(uniforms.texture0, 0);

        gl.activeTexture(gl.TEXTURE1);
        gl.bindTexture(gl.TEXTURE_2D, texture1);
        gl.uniform1i(uniforms.texture1, 1);

        const mMat = m.create();
        const mvpMat = m.create();

        let count = 0;
        function update() {
            clear();

            count++;
            const rad = ((count % 360) * Math.PI) / 180;
            const x = Math.cos(rad) * 1.25;

            for (const [y, destBlend, equation] of [
                [2.5, gl.ONE_MINUS_SRC_ALPHA, gl.FUNC_ADD],
                [0.0, gl.ONE, gl.FUNC_ADD],
                [-2.5, gl.ONE_MINUS_SRC_ALPHA, gl.FUNC_SUBTRACT],
            ]) {
                gl.blendFuncSeparate(gl.SRC_ALPHA, destBlend, gl.ONE, gl.ONE);
                gl.blendEquationSeparate(equation, gl.FUNC_ADD);

                // Render textures
                {
                    m.translate(m.identity(mMat), [x, y, 0], mMat);
                    m.multiply(vpMat, mMat, mvpMat);
                    gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                    gl.uniform1i(uniforms.useTexture, 1);

                    // Draw triangles based on the index buffer.
                    gl.drawElements(
                        gl.TRIANGLES,
                        indices.length,
                        /* type of index */ gl.UNSIGNED_SHORT,
                        /* start offset */ 0,
                    );
                }

                // Render translucent cover
                {
                    m.translate(m.identity(mMat), [1.25, y, 0], mMat);
                    m.multiply(vpMat, mMat, mvpMat);
                    gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                    gl.uniform1i(uniforms.useTexture, 0);

                    // Draw triangles based on the index buffer.
                    gl.drawElements(
                        gl.TRIANGLES,
                        indices.length,
                        /* type of index */ gl.UNSIGNED_SHORT,
                        /* start offset */ 0,
                    );
                }
            }

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
