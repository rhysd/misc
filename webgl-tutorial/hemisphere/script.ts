(function () {
    type Color = [number, number, number, number];

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    canvas.width = 512;
    canvas.height = 512;

    const gl = canvas.getContext('webgl')!;
    const m = new matIV();
    const q = new qtnIV();

    const skyButton = document.getElementById('sky')! as HTMLInputElement;

    function getSkyColor(): Color {
        const color = skyButton.value;
        const r = parseInt(color.slice(1, 3), 16) / 255;
        const g = parseInt(color.slice(3, 5), 16) / 255;
        const b = parseInt(color.slice(5, 7), 16) / 255;
        return [r, g, b, 1.0];
    }

    function clear(color: Color): void {
        const [r, g, b, a] = color.map(x => Math.min(x + 0.2, 1.0));
        gl.clearColor(r, g, b, a);
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

    function torus(row: number, col: number, innerRadius: number, outerRadius: number, c: Color): ObjectData {
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

                color.push(...c);
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
            q.toVecIII([0, 0, 3], this.q, this.pos);
            return this.pos;
        }

        up(): Vec3 {
            q.toVecIII([0, 1, 0], this.q, this.upDir);
            return this.upDir;
        }
    }

    async function main(): Promise<void> {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);

        // Enable culling
        gl.enable(gl.CULL_FACE);
        // Enable depth test
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);

        const camera = new Camera(canvas);
        const lightDirection: Vec3 = [-0.577, 0.577, 0.577];
        const skyDirection: Vec3 = [0, 1, 0];
        const groundColor: Color = [0.3, 0.2, 0.1, 1];

        const prog = new Program(vs, fs);
        prog.defineAttribute('position', 3);
        prog.defineAttribute('color', 4);
        prog.defineAttribute('normal', 3);
        prog.declareUniforms(
            'mvpMat',
            'invMat',
            'lightDirection',
            'eyePosition',
            'isGround',
            'skyDirection',
            'skyColor',
            'groundColor',
        );
        prog.use();

        gl.uniform3fv(prog.uniform('lightDirection'), lightDirection);
        gl.uniform3fv(prog.uniform('skyDirection'), skyDirection);
        gl.uniform4fv(prog.uniform('groundColor'), groundColor);

        const torusObject = prog.createObject(torus(64, 64, 0.15, 0.3, [0.7, 0.7, 0.7, 1.0]));
        const rectObject = prog.createObject(rect(4, groundColor));

        const mMat = m.create();
        const vMat = m.create();
        const pMat = m.create();
        const vpMat = m.create();
        const mvpMat = m.create();
        const invMat = m.create();

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
            const skyColor = getSkyColor();

            clear(skyColor);

            const cameraPos = camera.position();
            m.lookAt(cameraPos, /* Camera center */ [0, 0, 0], camera.up(), vMat);
            m.multiply(pMat, vMat, vpMat);
            gl.uniform3fv(prog.uniform('eyePosition'), cameraPos);
            gl.uniform4fv(prog.uniform('skyColor'), skyColor);

            function draw(obj: RenderObject): void {
                bindObjectBuffers(obj);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(prog.uniform('mvpMat'), /* transpose */ false, mvpMat);
                m.inverse(mMat, invMat);
                gl.uniformMatrix4fv(prog.uniform('invMat'), /* transpose */ false, invMat);
                gl.drawElements(gl.TRIANGLES, obj.lenIndices, gl.UNSIGNED_SHORT, 0);
            }

            // Render the object
            m.identity(mMat);
            m.rotate(mMat, rad, /* axis */ [0, 1, 0], mMat);
            m.rotate(mMat, Math.PI * 0.5, [1, 0, 0], mMat);
            gl.uniform1i(prog.uniform('isGround'), 0);
            draw(torusObject);

            // Render the ground
            m.identity(mMat);
            m.rotate(mMat, Math.PI * 1.5, [1, 0, 0], mMat);
            m.translate(mMat, [0, 0, -0.5], mMat);
            gl.uniform1i(prog.uniform('isGround'), 1);
            draw(rectObject);

            window.requestAnimationFrame(update);
        }

        update();
    }

    main().catch(err => alert(err.stack ?? err.message ?? String(err)));
})();
