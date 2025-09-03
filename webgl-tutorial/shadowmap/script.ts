(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 512;
    canvas.height = 512;
    const lightScaleInput = document.getElementById('light-scale')! as HTMLInputElement;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();
    const q = new qtnIV();

    function clear(color: Color): void {
        gl.clearColor(...color);
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

    function bindObjectBuffers(object: RenderObject): void {
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
            0, 2, 1,
            3, 1, 2,
        ];
        return { positions, normals, colors, indices };
    }

    interface OfflineFrameBuffer {
        frame: WebGLFramebuffer;
        depth: WebGLRenderbuffer;
        texture: WebGLTexture;
        width: number;
        height: number;
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

        return { frame, depth, texture, width, height };
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.enable(gl.CULL_FACE);

        const prog = createProgram(vs, fs);

        const torusObject = createObject(prog, torus(64, 64, 1, 2, [1, 1, 1, 1]));
        const rectObject = createObject(prog, rect(1, [0.7, 0.7, 0.7, 1]));

        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const vLightMat = m.create();
        const pLightMat = m.create(); // Projection matrix looking from the light position

        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 150,
            pMat,
        );
        m.perspective(90, 1.0, 0.1, 150, pLightMat);

        const cameraPos: Vec3 = [0, 0, 0];
        const cameraUp: Vec3 = [0, 0, 0];
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

        const uniforms = [
            'isShadow',
            'mMat',
            'mvpMat',
            'invMat',
            'tMat',
            'mvpLightMat',
            'lightPosition',
            'texture',
        ].reduce(
            (acc, name) => {
                acc[name] = gl.getUniformLocation(prog, name)!;
                return acc;
            },
            {} as Record<string, WebGLUniformLocation>,
        );

        const mvpMat = m.create();
        const invMat = m.create();
        const vpLightMat = m.create(); // View projection matrix for light
        const mvpLightMat = m.create();
        const lightUpDirection: Vec3 = [0, 0, -1];
        // Use the large canvas size to render shadows in higher resolution
        const frameBuf = createOfflineFrameBuffer(canvas.width * 4, canvas.height * 4);

        let count = 0;
        function update(): void {
            count++;

            // Calculate the view projection matrix
            q.toVecIII([-24, 62, 19], qCamera, cameraPos);
            q.toVecIII([0.9, 0.3, 0.3], qCamera, cameraUp);
            m.lookAt(cameraPos, /* Camera center */ [0, 0, 0], cameraUp, vMat);
            m.multiply(pMat, vMat, vpMat);

            // Calculate the view projection matrix for light
            const lightPos: Vec3 = [0, parseFloat(lightScaleInput.value), 0];
            m.lookAt(lightPos, [0, 0, 0], lightUpDirection, vLightMat);
            m.multiply(pLightMat, vLightMat, vpLightMat);

            // Prepare model transoformation matrices
            const torusModelMats = [];
            for (let i = 0; i < 10; i++) {
                const rad = (((count + i * 36) % 360) * Math.PI) / 180;
                const rad2 = ((((i % 5) * 72) % 360) * Math.PI) / 180;
                const ifl = -Math.floor(i / 5) + 1;
                const mMat = m.identity(m.create());
                m.rotate(mMat, rad2, [0.0, 1.0, 0.0], mMat);
                m.translate(mMat, [0.0, ifl * 10.0 + 10.0, (ifl - 2.0) * 7.0], mMat);
                m.rotate(mMat, rad, [1.0, 1.0, 0.0], mMat);
                torusModelMats.push(mMat);
            }
            const rectModelMat = m.identity(m.create());
            m.translate(rectModelMat, [0.0, -10.0, 0.0], rectModelMat);
            m.scale(rectModelMat, [30.0, 0.0, 30.0], rectModelMat);

            // Render the shadow mapping with depth buffer in an offline frame buffer
            {
                gl.bindFramebuffer(gl.FRAMEBUFFER, frameBuf.frame);

                clear([1.0, 1.0, 1.0, 1.0]);
                gl.viewport(0, 0, frameBuf.width, frameBuf.height);
                gl.uniform1i(uniforms.isShadow, 1);

                function draw(object: RenderObject, mMats: Float32Array[]): void {
                    bindObjectBuffers(object);
                    for (const mMat of mMats) {
                        m.multiply(vpLightMat, mMat, mvpLightMat);
                        gl.uniformMatrix4fv(uniforms.mvpLightMat, false, mvpLightMat);
                        gl.drawElements(gl.TRIANGLES, object.lenIndices, gl.UNSIGNED_SHORT, 0);
                    }
                }

                draw(torusObject, torusModelMats);
                draw(rectObject, [rectModelMat]);
            }

            // Render the canvas
            {
                gl.bindFramebuffer(gl.FRAMEBUFFER, null);

                // Bind the texture rendered on the frame buffer online to TEXTURE0
                gl.activeTexture(gl.TEXTURE0);
                gl.bindTexture(gl.TEXTURE_2D, frameBuf.texture);
                gl.uniform1i(uniforms.texture, 0);

                clear([0.5, 0.7, 1.0, 1.0]);
                gl.viewport(0, 0, canvas.width, canvas.height);
                gl.uniform1i(uniforms.isShadow, 0);

                gl.uniform3fv(uniforms.lightPosition, lightPos);

                // Calculate the texture projection matrix
                // prettier-ignore
                const tMat = new Float32Array([
                    0.5, 0.0, 0.0, 0.0,
                    0.0, 0.5, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.5, 0.5, 0.0, 1.0,
                ]);
                m.multiply(tMat, vpLightMat, tMat);
                gl.uniformMatrix4fv(uniforms.tMat, /* transpose */ false, tMat);

                function draw(object: RenderObject, mMats: Float32Array[]): void {
                    bindObjectBuffers(object);
                    for (const mMat of mMats) {
                        m.multiply(vpMat, mMat, mvpMat);
                        m.inverse(mMat, invMat);
                        m.multiply(vpLightMat, mMat, mvpLightMat);
                        gl.uniformMatrix4fv(uniforms.mMat, false, mMat);
                        gl.uniformMatrix4fv(uniforms.mvpMat, false, mvpMat);
                        gl.uniformMatrix4fv(uniforms.invMat, false, invMat);
                        gl.uniformMatrix4fv(uniforms.mvpLightMat, false, mvpLightMat);
                        gl.drawElements(gl.TRIANGLES, object.lenIndices, gl.UNSIGNED_SHORT, 0);
                    }
                }

                draw(torusObject, torusModelMats);
                draw(rectObject, [rectModelMat]);

                // The frame buffer will be used on the next iteration. Texture in frame buffer cannot be active.
                gl.bindTexture(gl.TEXTURE_2D, null);
            }

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch((err: Error) => alert(err.stack ?? err.message ?? String(err)));
})();
