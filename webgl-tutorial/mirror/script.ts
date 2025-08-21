(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 512;
    canvas.height = 512;

    const alphaInput = document.getElementById('alpha')! as HTMLInputElement;

    const gl = canvas.getContext('webgl', { stencil: true })!;
    const m = new matIV();
    const q = new qtnIV();

    function clear(): void {
        gl.clearColor(0.5, 0.7, 1.0, 1.0);
        gl.clearDepth(1.0);
        gl.clearStencil(0.0);
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

    interface RenderObject {
        attrs: Attribute[];
        ibo: WebGLBuffer;
        lenIndices: number;
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

    interface ObjectData {
        indices: number[];
        [key: string]: number[] | undefined;
    }

    interface BoundAttribute {
        loc: number;
        vbo: WebGLBuffer;
        stride: number;
    }

    interface AttributeDef {
        name: string;
        stride: number;
    }

    class Program {
        private prog: WebGLProgram;
        private attrLocs: Record<string, number>;
        private uniforms: Record<string, WebGLUniformLocation>;
        private attrDefs: AttributeDef[];

        constructor(vs: WebGLShader, fs: WebGLShader) {
            const p = gl.createProgram();

            gl.attachShader(p, vs);
            gl.attachShader(p, fs);
            gl.linkProgram(p);
            if (!gl.getProgramParameter(p, gl.LINK_STATUS)) {
                throw new Error(`Could not craete program: ${gl.getProgramInfoLog(p)}`);
            }
            gl.useProgram(p);

            this.prog = p;
            this.attrLocs = {};
            this.uniforms = {};
            this.attrDefs = [];
        }

        use(): void {
            gl.useProgram(this.prog);
        }

        defineAttribute(name: string, stride: number): void {
            this.attrDefs.push({ name, stride });
        }

        getAttributeLocation(name: string): number {
            let loc = this.attrLocs[name];
            if (loc === undefined) {
                loc = gl.getAttribLocation(this.prog, name);
                this.attrLocs[name] = loc;
            }
            return loc;
        }

        createAttribute(name: string, data: number[], stride: number): BoundAttribute {
            const loc = this.getAttributeLocation(name);
            const vbo = createVertexBuffer(data);
            return { loc, vbo, stride };
        }

        createObject(data: ObjectData): RenderObject {
            const attrs = this.attrDefs.map(def => {
                const v = data[def.name];
                if (v === undefined) {
                    throw new Error(`"${def.name}" is not a key of object data: ${JSON.stringify(data)}`);
                }
                return this.createAttribute(def.name, v, def.stride);
            });
            const ibo = createIndexBuffer(data.indices);
            const lenIndices = data.indices.length;
            return { attrs, ibo, lenIndices };
        }

        declareUniforms(...names: string[]): void {
            for (const name of names) {
                this.uniforms[name] = gl.getUniformLocation(this.prog, name)!;
            }
        }

        uniform(name: string): WebGLUniformLocation {
            const loc = this.uniforms[name];
            if (loc === undefined) {
                throw new Error(`Unknown uniform variable: ${name}`);
            }
            return loc;
        }
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
        const position = [];
        const normal = [];
        const color = [];
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
                position.push(x, y, z);

                const rx = rr * Math.cos(rad);
                const rz = rr * Math.sin(rad);
                normal.push(rx, ry, rz);

                color.push(...hsva((360 / col) * j, 1, 1, 1));
            }
        }

        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + col + 1, r + 1);
                indices.push(r + col + 1, r + col + 2, r + 1);
            }
        }

        return { position, normal, color, indices };
    }

    function sphere(row: number, col: number, radius: number): ObjectData {
        const position = [];
        const normal = [];
        const color = [];
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
                position.push(x, y, z);

                const rx = rr * Math.cos(rad);
                const rz = rr * Math.sin(rad);
                normal.push(rx, ry, rz);

                color.push(...hsva((360 / row) * i, 1, 1, 1));
            }
        }

        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + 1, r + col + 2);
                indices.push(r, r + col + 2, r + col + 1);
            }
        }

        return { position, normal, color, indices };
    }

    function rect(size: number, c: Color): ObjectData {
        const half = size / 2;
        // prettier-ignore
        const position = [
            -half,  half, 0.0,
             half,  half, 0.0,
            -half, -half, 0.0,
             half, -half, 0.0,
        ];
        // prettier-ignore
        const normal = [
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
        ];
        const color = [...c, ...c, ...c, ...c];
        // prettier-ignore
        const texCoord = [
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0
        ];
        // prettier-ignore
        const indices = [
            0, 2, 1,
            2, 3, 1,
        ];
        return { position, normal, color, texCoord, indices };
    }

    class Camera {
        private pos: Vec3;
        private upDir: Vec3;
        private q: Float32Array;

        constructor(canvas: HTMLCanvasElement) {
            this.pos = [0, 0, 0];
            this.upDir = [0, 0, 0];
            this.q = q.identity(q.create());

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
                    q.rotate(rad, [normY, normX, 0], this.q);
                },
                { passive: true },
            );

            canvas.addEventListener(
                'mouseleave',
                () => {
                    q.identity(this.q);
                },
                { passive: true },
            );
        }

        position(): Vec3 {
            q.toVecIII([0, 5, 5], this.q, this.pos);
            return this.pos;
        }

        up(): Vec3 {
            q.toVecIII([0, 1, -1], this.q, this.upDir);
            return this.upDir;
        }
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
        const [vs, fs, mvs, mfs] = await Promise.all([
            loadShader('shader.vert'),
            loadShader('shader.frag'),
            loadShader('mirror.vert'),
            loadShader('mirror.frag'),
        ]);

        // Enable culling
        gl.enable(gl.CULL_FACE);
        // Enable depth test
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        // Enable blending because we blend the mirror texture and the board
        gl.enable(gl.BLEND);
        gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE);
        gl.blendEquationSeparate(gl.FUNC_ADD, gl.FUNC_ADD);

        const prog = new Program(vs, fs);
        prog.defineAttribute('position', 3);
        prog.defineAttribute('color', 4);
        prog.defineAttribute('normal', 3);
        prog.declareUniforms('mMat', 'vpMat', 'invMat', 'lightDirection', 'eyePosition', 'isMirror');

        const torusObject = prog.createObject(torus(64, 64, 0.1, 0.4));
        const sphereObject = prog.createObject(sphere(64, 64, 0.25));
        const rectObject = prog.createObject(rect(4, [0.3, 0.3, 0.3, 1]));

        const mirrorProg = new Program(mvs, mfs);
        mirrorProg.defineAttribute('position', 3);
        mirrorProg.defineAttribute('texCoord', 2);
        mirrorProg.declareUniforms('ortMat', 'texture', 'alpha');
        const mirrorRect = mirrorProg.createObject(rect(4, [0, 0, 0, 1]));

        const camera = new Camera(canvas);
        const lightDirection: Vec3 = [-0.577, 0.577, 0.577];
        const fbuf = createOfflineFrameBuffer(canvas.width, canvas.height);

        const mMat = m.create();
        const vMat = m.create();
        const pMat = m.create();
        const vpMat = m.create();
        const invMat = m.create();
        const ortMat = m.create();

        m.lookAt(/*cam pos*/ [0, 0, 0.5], /*cam center*/ [0, 0, 0], /*cam up*/ [0, 1, 0], vMat);
        m.ortho(-2, 2, 2, -2, 0.1, 1, pMat);
        m.multiply(pMat, vMat, ortMat);
        m.perspective(
            /* fov */ 45,
            /* aspect ratio */ canvas.width / canvas.height,
            /* near clip */ 0.1,
            /* far clip */ 10,
            pMat,
        );

        let count = 0;
        function update(): void {
            count++;
            const rad = ((count % 720) * Math.PI) / 360;
            const upDown = Math.sin(rad) * 0.25;

            {
                function draw(obj: RenderObject): void {
                    bindObjectBuffers(obj);
                    gl.uniformMatrix4fv(prog.uniform('mMat'), /* transpose */ false, mMat);
                    m.inverse(mMat, invMat);
                    gl.uniformMatrix4fv(prog.uniform('invMat'), /* transpose */ false, invMat);
                    gl.drawElements(gl.TRIANGLES, obj.lenIndices, gl.UNSIGNED_SHORT, 0);
                }

                function drawTorus(): void {
                    m.identity(mMat);
                    m.rotate(mMat, rad, /* axis */ [0, 1, 0], mMat);
                    m.translate(mMat, [0, 0.75 + upDown, 0], mMat);
                    m.rotate(mMat, Math.PI * 0.5, [1, 0, 0], mMat);
                    draw(torusObject);
                }

                function drawSphere(): void {
                    m.identity(mMat);
                    m.rotate(mMat, -rad, /* axis */ [0, 1, 0], mMat);
                    m.translate(mMat, [0, 0.75, 1], mMat);
                    draw(sphereObject);
                }

                prog.use();
                gl.disable(gl.STENCIL_TEST);

                const cameraPos = camera.position();
                m.lookAt(cameraPos, /* Camera center */ [0, 0, 0], camera.up(), vMat);
                m.multiply(pMat, vMat, vpMat);
                gl.uniformMatrix4fv(prog.uniform('vpMat'), /* transpose */ false, vpMat);

                gl.uniform3fv(prog.uniform('lightDirection'), lightDirection);
                gl.uniform3fv(prog.uniform('eyePosition'), cameraPos);

                // 1. Render the mirror world

                gl.bindFramebuffer(gl.FRAMEBUFFER, fbuf.frame);

                clear();
                gl.cullFace(gl.FRONT); // We are drawing mirror world from opposite side so culling face should be flipped
                gl.uniform1i(prog.uniform('isMirror'), 1);

                drawTorus();
                drawSphere();

                gl.bindFramebuffer(gl.FRAMEBUFFER, null);

                // 2. Render the real world

                clear();
                gl.cullFace(gl.BACK); // Restore the culling face to the default
                gl.uniform1i(prog.uniform('isMirror'), 0);

                drawTorus();
                drawSphere();

                // Render the board
                {
                    // Write `1` to the pixels inside the board to avoid writing mirror world outside the board
                    gl.enable(gl.STENCIL_TEST);
                    gl.stencilFunc(gl.ALWAYS, 1, ~0);
                    gl.stencilOp(gl.KEEP, gl.KEEP, gl.REPLACE);

                    m.identity(mMat);
                    m.rotate(mMat, Math.PI * 1.5, [1, 0, 0], mMat);
                    draw(rectObject);
                }
            }

            // 3. Compose mirror world rendered to the frame buffer to the real world
            {
                mirrorProg.use();

                const alpha = parseFloat(alphaInput.value);

                // Only write the mirror world where the stencil value is `1`
                gl.stencilFunc(gl.EQUAL, 1, ~0);
                gl.stencilOp(gl.KEEP, gl.KEEP, gl.KEEP);

                gl.activeTexture(gl.TEXTURE0);
                gl.bindTexture(gl.TEXTURE_2D, fbuf.texture);

                bindObjectBuffers(mirrorRect);
                gl.uniformMatrix4fv(mirrorProg.uniform('ortMat'), false, ortMat);
                gl.uniform1i(mirrorProg.uniform('texture'), 0);
                gl.uniform1f(mirrorProg.uniform('alpha'), alpha);
                gl.drawElements(gl.TRIANGLES, mirrorRect.lenIndices, gl.UNSIGNED_SHORT, 0);

                gl.bindTexture(gl.TEXTURE_2D, null);
            }

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message));
})();
