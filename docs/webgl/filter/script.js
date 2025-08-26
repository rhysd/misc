"use strict";
(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 512;
    canvas.height = 512;
    const grayButton = document.getElementById('grayscale');
    const sobelButton = document.getElementById('sobel');
    const laplacianButton = document.getElementById('laplacian');
    const gaussianButton = document.getElementById('gaussian');
    const gaussianWeightInput = document.getElementById('gaussian-weight');
    const gl = canvas.getContext('webgl');
    const m = new matIV();
    const q = new qtnIV();
    function clear(color) {
        gl.clearColor(...color);
        gl.clearDepth(1.0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    }
    async function loadShader(path) {
        const res = await fetch(path);
        if (!res.ok) {
            throw new Error(`Fetching ${path} failed with status ${res.status}: ${res.statusText}`);
        }
        const src = await res.text();
        let shader;
        if (path.endsWith('.vert')) {
            shader = gl.createShader(gl.VERTEX_SHADER);
        }
        else if (path.endsWith('.frag')) {
            shader = gl.createShader(gl.FRAGMENT_SHADER);
        }
        else {
            throw new Error(`Unknown file extension for shader: ${path}`);
        }
        if (!shader) {
            throw new Error(`Shader could not be created for ${path}`);
        }
        gl.shaderSource(shader, src);
        gl.compileShader(shader);
        if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            return shader;
        }
        else {
            throw new Error(`Shader compilation failed: ${gl.getShaderInfoLog(shader)}`);
        }
    }
    function createVertexBuffer(data) {
        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ARRAY_BUFFER, null);
        return vbo;
    }
    function createIndexBuffer(data) {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }
    class Program {
        prog;
        attrLocs;
        uniforms;
        attrDefs;
        constructor(vs, fs) {
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
        use() {
            gl.useProgram(this.prog);
        }
        defineAttribute(name, dataName, stride) {
            this.attrDefs.push({ name, dataName, stride });
        }
        getAttributeLocation(name) {
            let loc = this.attrLocs[name];
            if (loc === undefined) {
                loc = gl.getAttribLocation(this.prog, name);
                this.attrLocs[name] = loc;
            }
            return loc;
        }
        createAttribute(name, data, stride) {
            const loc = this.getAttributeLocation(name);
            const vbo = createVertexBuffer(data);
            return { loc, vbo, stride };
        }
        createObject(data) {
            const attrs = this.attrDefs.map(def => {
                return this.createAttribute(def.name, data[def.dataName], def.stride);
            });
            const ibo = createIndexBuffer(data.indices);
            const lenIndices = data.indices.length;
            return { attrs, ibo, lenIndices };
        }
        declareUniforms(...names) {
            for (const name of names) {
                this.uniforms[name] = gl.getUniformLocation(this.prog, name);
            }
        }
        uniform(name) {
            const loc = this.uniforms[name];
            if (loc === undefined) {
                throw new Error(`Unknown uniform variable: ${name}`);
            }
            return loc;
        }
    }
    function bindObjectBuffers(object) {
        for (const attr of object.attrs) {
            const { loc, vbo, stride } = attr;
            gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
            gl.enableVertexAttribArray(loc);
            gl.vertexAttribPointer(loc, stride, gl.FLOAT, false, 0, 0);
        }
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, object.ibo);
    }
    function hsva(h, s, v, a) {
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
    function torus(row, col, innerRadius, outerRadius, color) {
        const positions = [];
        const normals = [];
        const colors = [];
        const texCoords = [];
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
                const s = j / col;
                let t = i / row + 0.5;
                if (t > 1) {
                    t -= 1;
                }
                t = 1 - t;
                texCoords.push(s, t);
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
        return { positions, normals, colors, texCoords, indices };
    }
    function rect(size, color) {
        const half = size / 2;
        // prettier-ignore
        const positions = [
            -half, half, 0.0,
            half, half, 0.0,
            -half, -half, 0.0,
            half, -half, 0.0,
        ];
        // prettier-ignore
        const normals = [
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
        ];
        const colors = [...color, ...color, ...color, ...color];
        // prettier-ignore
        const texCoords = [
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
        return { positions, normals, colors, texCoords, indices };
    }
    class Camera {
        pos;
        upDir;
        q;
        constructor(canvas) {
            this.pos = [0, 0, 0];
            this.upDir = [0, 0, 0];
            this.q = q.identity(q.create());
            canvas.addEventListener('mousemove', event => {
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
            }, { passive: true });
            canvas.addEventListener('mouseleave', () => {
                q.identity(this.q);
            }, { passive: true });
        }
        position() {
            q.toVecIII([0, 20, 0], this.q, this.pos);
            return this.pos;
        }
        up() {
            q.toVecIII([0, 0, -1], this.q, this.upDir);
            return this.upDir;
        }
    }
    function createOfflineFrameBuffer(width, height) {
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
        gl.texImage2D(gl.TEXTURE_2D, mipmapLevel, gl.RGBA, width, height, 
        /* width of border */ 0, gl.RGBA, gl.UNSIGNED_BYTE, 
        /* texture data is unbound*/ null);
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
    function gaussianWeight(strength) {
        const weight = new Array(10);
        const d = strength * strength;
        let t = 0.0;
        for (let i = 0; i < weight.length; i++) {
            const r = 1.0 + 2.0 * i;
            let w = Math.exp((-0.5 * (r * r)) / d);
            weight[i] = w;
            if (i > 0) {
                w *= 2.0;
            }
            t += w;
        }
        for (let i = 0; i < weight.length; i++) {
            weight[i] /= t;
        }
        return weight;
    }
    async function main() {
        const [vs, fs, filterVs, filterFs] = await Promise.all([
            loadShader('shader.vert'),
            loadShader('shader.frag'),
            loadShader('filter.vert'),
            loadShader('filter.frag'),
        ]);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        gl.enable(gl.CULL_FACE);
        const prog = new Program(vs, fs);
        prog.defineAttribute('position', 'positions', 3);
        prog.defineAttribute('color', 'colors', 4);
        prog.defineAttribute('normal', 'normals', 3);
        prog.declareUniforms('mvpMat', 'invMat', 'lightDirection', 'eyePosition', 'ambientColor');
        gl.uniform3fv(prog.uniform('lightDirection'), [-0.577, 0.577, 0.577]);
        const filterProg = new Program(filterVs, filterFs);
        filterProg.defineAttribute('position', 'positions', 3);
        filterProg.defineAttribute('texCoord', 'texCoords', 2);
        filterProg.declareUniforms('mvpMat', 'texture', 'filter', 'canvasHeight', 'filterKernel', 'gaussianWeight', 'gaussianIsHorizontal');
        gl.uniform1i(filterProg.uniform('texture'), 0);
        gl.uniform1f(filterProg.uniform('canvasHeight'), canvas.height);
        const torusObject = prog.createObject(torus(64, 64, 1, 2, [1, 1, 1, 1]));
        const rectObject = filterProg.createObject(rect(2, [1, 1, 1, 1]));
        const mMat = m.create();
        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const mvpMat = m.create();
        const invMat = m.create();
        const camera = new Camera(canvas);
        // Use the large canvas size to render shadows in higher resolution
        const frameBufs = [
            createOfflineFrameBuffer(canvas.width, canvas.height),
            createOfflineFrameBuffer(canvas.width, canvas.height), // For gaussian filter
        ];
        let count = 0;
        function update() {
            count++;
            const GRAYSCALE_FILTER = 1;
            const SOBEL_FILTER = 2;
            const LAPLACIAN_FILTER = 3;
            const GAUSSIAN_FILTER = 4;
            const filter = gaussianButton.checked
                ? GAUSSIAN_FILTER
                : laplacianButton.checked
                    ? LAPLACIAN_FILTER
                    : sobelButton.checked
                        ? SOBEL_FILTER
                        : grayButton.checked
                            ? GRAYSCALE_FILTER
                            : 0;
            // Render the scene to the frame buffer
            {
                prog.use();
                gl.bindFramebuffer(gl.FRAMEBUFFER, frameBufs[0].frame);
                clear(hsva(Math.floor(count / 2) % 360, 0.5, 1, 1));
                const rad = ((count % 360) * Math.PI) / 180;
                const eyePos = camera.position();
                m.lookAt(eyePos, /* Camera center */ [0, 0, 0], camera.up(), vMat);
                m.perspective(
                /* fov */ 90, 
                /* aspect ratio */ canvas.width / canvas.height, 
                /* near clip */ 0.1, 
                /* far clip */ 100, pMat);
                m.multiply(pMat, vMat, vpMat);
                bindObjectBuffers(torusObject);
                for (let i = 0; i < 9; i++) {
                    m.identity(mMat);
                    m.rotate(mMat, (i * 2 * Math.PI) / 9, [0, 1, 0], mMat);
                    m.translate(mMat, [0.0, 0.0, 10.0], mMat);
                    m.rotate(mMat, rad, [1, 1, 0], mMat);
                    m.multiply(vpMat, mMat, mvpMat);
                    m.inverse(mMat, invMat);
                    gl.uniformMatrix4fv(prog.uniform('mvpMat'), false, mvpMat);
                    gl.uniformMatrix4fv(prog.uniform('invMat'), false, invMat);
                    gl.uniform3fv(prog.uniform('eyePosition'), eyePos);
                    gl.uniform4fv(prog.uniform('ambientColor'), hsva(i * 40, 1, 1, 1));
                    gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, 
                    /* type of index */ gl.UNSIGNED_SHORT, 
                    /* start offset */ 0);
                }
            }
            // Render the frame buffer texture to the canvas applying the filter
            {
                filterProg.use();
                gl.bindFramebuffer(gl.FRAMEBUFFER, null);
                clear([0, 0, 0, 1]);
                // Orthographic projection
                m.lookAt([0, 0, 0.5], [0, 0, 0], [0, 1, 0], vMat);
                m.ortho(-1, 1, 1, -1, 0.1, 1, pMat);
                m.multiply(pMat, vMat, vpMat);
                gl.activeTexture(gl.TEXTURE0);
                gl.bindTexture(gl.TEXTURE_2D, frameBufs[0].texture);
                bindObjectBuffers(rectObject);
                gl.uniformMatrix4fv(filterProg.uniform('mvpMat'), false, vpMat);
                gl.uniform1i(filterProg.uniform('filter'), filter);
                switch (filter) {
                    case SOBEL_FILTER: {
                        // prettier-ignore
                        const horizontalKernel = [
                            1.0, 0.0, -1.0,
                            2.0, 0.0, -2.0,
                            1.0, 0.0, -1.0
                        ];
                        gl.uniform1fv(filterProg.uniform('filterKernel'), horizontalKernel);
                        break;
                    }
                    case LAPLACIAN_FILTER: {
                        // prettier-ignore
                        const kernel = [
                            1.0, 1.0, 1.0,
                            1.0, -8.0, 1.0,
                            1.0, 1.0, 1.0
                        ];
                        gl.uniform1fv(filterProg.uniform('filterKernel'), kernel);
                        break;
                    }
                    case GAUSSIAN_FILTER: {
                        const w = gaussianWeight(parseFloat(gaussianWeightInput.value));
                        gl.uniform1fv(filterProg.uniform('gaussianWeight'), w);
                        gl.uniform1i(filterProg.uniform('gaussianIsHorizontal'), 1);
                        // Apply horizontal gaussian filter (1st pass)
                        gl.bindFramebuffer(gl.FRAMEBUFFER, frameBufs[1].frame);
                        clear([0, 0, 0, 1]);
                        gl.drawElements(gl.TRIANGLES, rectObject.lenIndices, gl.UNSIGNED_SHORT, 0);
                        // Apply vertical gaussian filter (2nd pass)
                        gl.bindFramebuffer(gl.FRAMEBUFFER, null);
                        gl.bindTexture(gl.TEXTURE_2D, frameBufs[1].texture);
                        gl.uniform1i(filterProg.uniform('gaussianIsHorizontal'), 0);
                        break;
                    }
                    default:
                        break;
                }
                gl.drawElements(gl.TRIANGLES, rectObject.lenIndices, gl.UNSIGNED_SHORT, 0);
                gl.bindTexture(gl.TEXTURE_2D, null);
            }
            window.requestAnimationFrame(update);
        }
        update();
    }
    main().catch(err => alert(err.stack ?? err.message));
})();
//# sourceMappingURL=script.js.map