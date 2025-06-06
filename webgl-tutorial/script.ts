(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 512;
    canvas.height = 512;

    const blur = document.getElementById('blur')! as HTMLInputElement;

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

    interface RenderObject {
        attrs: Attribute[];
        ibo: WebGLBuffer;
        lenIndices: number;
    }

    interface ObjectData {
        positions: number[];
        normals: number[];
        colors: number[];
        indices: number[];
    }

    function createObject(prog: WebGLProgram, data: ObjectData): RenderObject {
        const { positions, normals, colors, indices } = data;
        return {
            attrs: [
                createAttribute('position', positions, 3, prog),
                createAttribute('normal', normals, 3, prog),
                createAttribute('color', colors, 4, prog),
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

    function bindRenderObject(object: RenderObject): void {
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

    function createRect(prog: WebGLProgram, size: number): RenderObject {
        // prettier-ignore
        const positions = [
               0, size, 0,
            size, size, 0,
               0,    0, 0,
            size,    0, 0,
        ];
        // prettier-ignore
        const textures = [
            0, 0,
            1, 0,
            0, 1,
            1, 1
        ];
        // prettier-ignore
        const indices = [
            0, 1, 2,
            3, 2, 1,
        ];
        return {
            attrs: [
                createAttribute('position', positions, 3, prog),
                createAttribute('textureCoord', textures, 2, prog),
            ],
            ibo: createIndexBuffer(indices),
            lenIndices: indices.length,
        };
    }

    interface OfflineFrameBuffer {
        frame: WebGLFramebuffer;
        depth: WebGLRenderbuffer;
        texture: WebGLTexture;
    }

    function createOfflineFrameBuffer(width: number, height: number): OfflineFrameBuffer {
        const frame = gl.createFramebuffer();
        gl.bindFramebuffer(gl.FRAMEBUFFER, frame);

        // Setup the render buffer as depth buffer
        const depth = gl.createRenderbuffer();
        gl.bindRenderbuffer(gl.RENDERBUFFER, depth);
        gl.renderbufferStorage(gl.RENDERBUFFER, gl.DEPTH_COMPONENT16, width, height);

        // Attach the depth buffer to the frame buffer
        gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, depth);

        // Create texture as the rendering target
        const mipmapLevel = 0;
        const texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(
            gl.TEXTURE_2D,
            mipmapLevel,
            gl.RGBA,
            width,
            height,
            /* width of border */ 0,
            gl.RGBA,
            gl.UNSIGNED_BYTE,
            /* texture data is unbound*/ null,
        );
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

        // Attach the texture to the frame buffer
        gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, mipmapLevel);

        // Ensure all buffers are unbound after creation
        gl.bindTexture(gl.TEXTURE_2D, null);
        gl.bindRenderbuffer(gl.RENDERBUFFER, null);
        gl.bindFramebuffer(gl.FRAMEBUFFER, null);

        return { frame, depth, texture };
    }

    function uniformLocations(prog: WebGLProgram, names: string[]): Record<string, WebGLUniformLocation> {
        return names.reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );
    }

    async function main(): Promise<void> {
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const [frameVS, frameFS, blurVS, blurFS] = await Promise.all([
            loadShader('frame.vert'),
            loadShader('frame.frag'),
            loadShader('blur.vert'),
            loadShader('blur.frag'),
        ]);

        const frameProg = createProgram(frameVS, frameFS);

        const torusObject = createObject(frameProg, torus(64, 64, 1.5, 3.0));

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const eyeDirection: [number, number, number] = [0, 0, 20];

        m.lookAt(/* eye position */ eyeDirection, /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 100,
            pMat,
        );
        m.multiply(pMat, vMat, vpMat);

        const uniforms = uniformLocations(frameProg, [
            'mvpMat',
            'mMat',
            'invMat',
            'lightPosition',
            'eyeDirection',
            'ambientColor',
            'texture',
        ]);

        gl.activeTexture(gl.TEXTURE0);
        gl.uniform1i(uniforms.texture, 0);

        const offline = createOfflineFrameBuffer(canvas.width, canvas.height);

        const mMat = m.create();
        const mvpMat = m.create();
        const invMat = m.create();
        const lightPosition = [10, 0, 0];
        const ambientColor = [0.1, 0.1, 0.1, 1.0];

        gl.uniform3fv(uniforms.lightPosition, lightPosition);
        gl.uniform3fv(uniforms.eyeDirection, eyeDirection);
        gl.uniform4fv(uniforms.ambientColor, ambientColor);

        // Create another program for rendering the rendered texture
        const blurProg = createProgram(blurVS, blurFS);
        const rectObject = createRect(blurProg, 2.0);
        const blurUniforms = uniformLocations(blurProg, ['mvpMat', 'texture', 'textureLength', 'blur']);
        gl.uniform1i(blurUniforms.texture, 0);
        gl.uniform1f(blurUniforms.textureLength, 512.0);

        let count = 0;
        function update() {
            count++;
            const rad = ((count % 360) * Math.PI) / 180;

            // Render a torus object on the offline frame buffer
            {
                // Switch shaders
                gl.useProgram(frameProg);

                // Bind the offline frame buffer
                gl.bindFramebuffer(gl.FRAMEBUFFER, offline.frame);

                // Unbind the texture because the render target cannot be bound while it is rendered
                gl.bindTexture(gl.TEXTURE_2D, null);

                // Clear the offline frame buffer
                clear();

                bindRenderObject(torusObject);

                m.identity(mMat);
                m.rotate(mMat, rad, /* axis */ [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);

                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);

                // Draw triangles based on the index buffer.
                gl.drawElements(
                    gl.TRIANGLES,
                    torusObject.lenIndices,
                    /* type of index */ gl.UNSIGNED_SHORT,
                    /* start offset */ 0,
                );

                // Unbind the offline frame buffer. Actual rendering to the offline frame buffer happens here
                gl.bindFramebuffer(gl.FRAMEBUFFER, null);
            }

            // Render the torus object rendered to the texture to the actual canvas multiple times
            {
                // Switch shaders
                gl.useProgram(blurProg);

                // Clear the main canvas
                clear();

                bindRenderObject(rectObject);

                // Bind the texture which is the rendering result of the offline frame buffer
                gl.bindTexture(gl.TEXTURE_2D, offline.texture);

                for (let y = -4; y < 4; y++) {
                    for (let x = -4; x < 4; x++) {
                        if (-2 <= x && x < 2 && -2 <= y && y < 2) {
                            continue;
                        }
                        m.identity(mMat);
                        m.translate(mMat, [x * 2.0, y * 2.0, 0], mMat);
                        m.multiply(vpMat, mMat, mvpMat);

                        gl.uniformMatrix4fv(blurUniforms.mvpMat, /* transpose */ false, mvpMat);
                        gl.uniform1f(blurUniforms.blur, 0.0);

                        // Draw triangles based on the index buffer.
                        gl.drawElements(gl.TRIANGLES, rectObject.lenIndices, gl.UNSIGNED_SHORT, 0);
                    }
                }

                m.identity(mMat);
                m.translate(mMat, [-4, -4, 0], mMat);
                m.scale(mMat, [4, 4, 0], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(blurUniforms.mvpMat, /* transpose */ false, mvpMat);
                console.log(parseFloat(blur.value));
                gl.uniform1f(blurUniforms.blur, parseFloat(blur.value));
                gl.drawElements(gl.TRIANGLES, rectObject.lenIndices, gl.UNSIGNED_SHORT, 0);
            }

            // Actual re-rendering happens here
            gl.flush();

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
