"use strict";
(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 600;
    canvas.height = 400;
    const gl = canvas.getContext('webgl', { stencil: true });
    const m = new matIV();
    const q = new qtnIV();
    function clear() {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
        gl.clearDepth(1.0);
        gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT | gl.STENCIL_BUFFER_BIT);
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
    function createProgram(vs, fs) {
        const program = gl.createProgram();
        gl.attachShader(program, vs);
        gl.attachShader(program, fs);
        gl.linkProgram(program);
        if (gl.getProgramParameter(program, gl.LINK_STATUS)) {
            gl.useProgram(program);
            return program;
        }
        else {
            throw new Error(`Could not craete program: ${gl.getProgramInfoLog(program)}`);
        }
    }
    function createVertexBuffer(data) {
        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ARRAY_BUFFER, null);
        return vbo;
    }
    function createAttribute(name, data, stride, program) {
        const loc = gl.getAttribLocation(program, name);
        const vbo = createVertexBuffer(data);
        return { loc, vbo, stride };
    }
    function createIndexBuffer(data) {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }
    function createObject(prog, data) {
        const { positions, normals, colors, indices, textures } = data;
        return {
            attrs: [
                createAttribute('position', positions, 3, prog),
                createAttribute('normal', normals, 3, prog),
                createAttribute('color', colors, 4, prog),
                createAttribute('textureCoord', textures, 2, prog),
            ],
            ibo: createIndexBuffer(indices),
            lenIndices: indices.length,
        };
    }
    function bindObjectToBuffers(object) {
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
    function torus(row, col, innerRadius, outerRadius) {
        const positions = [];
        const normals = [];
        const colors = [];
        const indices = [];
        const textures = [];
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
                const s = (1 / col) * j;
                let t = (1 / row) * i + 0.5;
                if (t > 1) {
                    t -= 1;
                }
                t = 1 - t;
                textures.push(s, t);
            }
        }
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + col + 1, r + 1);
                indices.push(r + col + 1, r + col + 2, r + 1);
            }
        }
        return { positions, normals, colors, indices, textures };
    }
    function sphere(row, col, radius, color) {
        const positions = [];
        const normals = [];
        const colors = [];
        const indices = [];
        const textures = [];
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
                colors.push(...color);
                textures.push(1 - (1 / col) * j, (1 / row) * i);
            }
        }
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + 1, r + col + 2);
                indices.push(r, r + col + 2, r + col + 1);
            }
        }
        return { positions, normals, colors, indices, textures };
    }
    function loadImage(src) {
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
    async function loadTexture2D(src) {
        const img = await loadImage(src);
        const tex = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, tex);
        gl.texImage2D(
        /* target */ gl.TEXTURE_2D, 
        /* level of mipmap */ 0, 
        /* color components in texture */ gl.RGBA, 
        /* format of the texel data*/ gl.RGBA, 
        /* 1 byte per element of RGBA */ gl.UNSIGNED_BYTE, img);
        gl.generateMipmap(gl.TEXTURE_2D);
        gl.bindTexture(gl.TEXTURE_2D, null);
        return tex;
    }
    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        const prog = createProgram(vs, fs);
        const torusObject = createObject(prog, torus(64, 64, 0.25, 1));
        const sphereObject = createObject(prog, sphere(64, 64, 1, [1, 1, 1, 1]));
        const uniforms = ['mvpMat', 'isOutline', 'texture', 'useTexture'].reduce((acc, name) => {
            acc[name] = gl.getUniformLocation(prog, name);
            return acc;
        }, {});
        const texture = await loadTexture2D('../assets/floor.png');
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.uniform1i(uniforms.texture, 0);
        const pMat = m.identity(m.create());
        m.perspective(
        /* fov */ 45, 
        /* aspect ratio */ canvas.width / canvas.height, 
        /* near clip */ 0.1, 
        /* far clip */ 100, pMat);
        const vMat = m.create();
        const vpMat = m.create();
        const mMat = m.create();
        const mvpMat = m.create();
        const qCamera = q.identity(q.create());
        const mouseMat = m.create();
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
            q.rotate(rad, [normY, normX, 0], qCamera);
        }, { passive: true });
        let count = 0;
        function update() {
            count++;
            const rad = ((count % 360) * Math.PI) / 180;
            clear();
            m.identity(mouseMat);
            q.toMatIV(qCamera, mouseMat);
            m.lookAt(/* eye position */ [0, 0, 10], /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
            m.multiply(vMat, mouseMat, vMat);
            m.multiply(pMat, vMat, vpMat);
            // The first path: Render the silhouette on depth buffer
            {
                gl.enable(gl.STENCIL_TEST);
                // Do not render colors
                gl.colorMask(/*R*/ false, /*G*/ false, /*B*/ false, /*A*/ false);
                // Do not write to depth buffer
                gl.depthMask(false);
                // Write to stencil buffer.
                // - Write 1 on the pixel where the model is rendered
                // - Write 0 on the pixel where the model is not rendered
                gl.stencilFunc(gl.ALWAYS, 1, ~0);
                gl.stencilOp(gl.KEEP, gl.REPLACE, gl.REPLACE);
                bindObjectToBuffers(torusObject);
                m.rotate(m.identity(mMat), rad, [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, false, mvpMat);
                gl.uniform1i(uniforms.useTexture, 0);
                gl.uniform1i(uniforms.isOutline, 1);
                gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            // The second path: Render background texture on large sphere
            {
                // Enable rendering colors and depth test
                gl.colorMask(/*R*/ true, /*G*/ true, /*B*/ true, /*A*/ true);
                gl.depthMask(true);
                // Render the texture only where the model is not rendered
                gl.stencilFunc(gl.EQUAL, 0, ~0);
                gl.stencilOp(gl.KEEP, gl.KEEP, gl.KEEP);
                bindObjectToBuffers(sphereObject);
                m.scale(m.identity(mMat), [50, 50, 50], mMat); // Render 50x sphere
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, false, mvpMat);
                gl.uniform1i(uniforms.useTexture, 1);
                gl.uniform1i(uniforms.isOutline, 0);
                gl.drawElements(gl.TRIANGLES, sphereObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            // The third path: Render the torus model on the silhouette
            {
                // Disable stencil test to render object without the effect of stencil buffer
                gl.disable(gl.STENCIL_TEST);
                bindObjectToBuffers(torusObject);
                m.rotate(m.identity(mMat), rad, [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, false, mvpMat);
                gl.uniform1i(uniforms.useTexture, 0);
                gl.uniform1i(uniforms.isOutline, 0);
                gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            // Actual re-rendering happens here
            gl.flush();
            window.requestAnimationFrame(update);
        }
        update();
    }
    main().catch(err => alert(err.stack ?? err.message));
})();
//# sourceMappingURL=script.js.map