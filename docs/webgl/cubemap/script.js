"use strict";
(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 600;
    canvas.height = 600;
    const reflectButton = document.getElementById('reflect-surface');
    const refractButton = document.getElementById('refract-surface');
    const gl = canvas.getContext('webgl');
    const m = new matIV();
    const q = new qtnIV();
    function clear() {
        gl.clearColor(0.0, 0.0, 0.0, 1.0);
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
    function createObject(prog, data) {
        const { positions, normals, indices } = data;
        return {
            attrs: [createAttribute('position', positions, 3, prog), createAttribute('normal', normals, 3, prog)],
            ibo: createIndexBuffer(indices),
            lenIndices: indices.length,
        };
    }
    function createIndexBuffer(data) {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
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
    function torus(row, col, innerRadius, outerRadius) {
        const positions = [];
        const normals = [];
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
            }
        }
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + col + 1, r + 1);
                indices.push(r + col + 1, r + col + 2, r + 1);
            }
        }
        return { positions, normals, indices };
    }
    function sphere(row, col, radius) {
        const positions = [];
        const normals = [];
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
            }
        }
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + 1, r + col + 2);
                indices.push(r, r + col + 2, r + col + 1);
            }
        }
        return { positions, normals, indices };
    }
    function cube(side) {
        const hs = side * 0.5;
        // prettier-ignore
        const positions = [
            -hs, -hs, hs, hs, -hs, hs, hs, hs, hs, -hs, hs, hs,
            -hs, -hs, -hs, -hs, hs, -hs, hs, hs, -hs, hs, -hs, -hs,
            -hs, hs, -hs, -hs, hs, hs, hs, hs, hs, hs, hs, -hs,
            -hs, -hs, -hs, hs, -hs, -hs, hs, -hs, hs, -hs, -hs, hs,
            hs, -hs, -hs, hs, hs, -hs, hs, hs, hs, hs, -hs, hs,
            -hs, -hs, -hs, -hs, -hs, hs, -hs, hs, hs, -hs, hs, -hs
        ];
        // prettier-ignore
        const normals = [
            -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0
        ];
        // prettier-ignore
        const indices = [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            8, 9, 10, 8, 10, 11,
            12, 13, 14, 12, 14, 15,
            16, 17, 18, 16, 18, 19,
            20, 21, 22, 20, 22, 23
        ];
        return { positions, normals, indices };
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
    async function loadCubeMapTexture(sources) {
        const map = await Promise.all(sources.map(async ([target, path]) => {
            const img = await loadImage(path);
            return [target, img];
        }));
        const tex = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_CUBE_MAP, tex);
        for (const [target, image] of map) {
            gl.texImage2D(
            /* target */ target, 
            /* level of mipmap */ 0, 
            /* color components in texture */ gl.RGBA, 
            /* format of the texel data*/ gl.RGBA, 
            /* 1 byte per element of RGBA */ gl.UNSIGNED_BYTE, image);
        }
        gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
        gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.bindTexture(gl.TEXTURE_CUBE_MAP, null);
        return tex;
    }
    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        const prog = createProgram(vs, fs);
        const cubeMapTex = await loadCubeMapTexture([
            [gl.TEXTURE_CUBE_MAP_POSITIVE_X, '../assets/cubemap/px.png'],
            [gl.TEXTURE_CUBE_MAP_POSITIVE_Y, '../assets/cubemap/py.png'],
            [gl.TEXTURE_CUBE_MAP_POSITIVE_Z, '../assets/cubemap/pz.png'],
            [gl.TEXTURE_CUBE_MAP_NEGATIVE_X, '../assets/cubemap/nx.png'],
            [gl.TEXTURE_CUBE_MAP_NEGATIVE_Y, '../assets/cubemap/ny.png'],
            [gl.TEXTURE_CUBE_MAP_NEGATIVE_Z, '../assets/cubemap/nz.png'],
        ]);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_CUBE_MAP, cubeMapTex);
        const torusObject = createObject(prog, torus(64, 64, 0.75, 1.75));
        const sphereObject = createObject(prog, sphere(64, 64, 2));
        const cubeObject = createObject(prog, cube(2.0));
        const cameraPos = [0, 0, 20];
        const cameraUp = [0, 1, 0];
        const qCamera = q.identity(q.create());
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
        const uniforms = ['mvpMat', 'mMat', 'eyePosition', 'envTexture', 'surface'].reduce((acc, name) => {
            acc[name] = gl.getUniformLocation(prog, name);
            return acc;
        }, {});
        gl.uniform1i(uniforms.envTexture, 0);
        const pMat = m.identity(m.create());
        m.perspective(
        /* fov */ 45, 
        /* aspect ratio */ canvas.width / canvas.height, 
        /* near clip */ 0.1, 
        /* far clip */ 200, pMat);
        const vMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const mMat = m.create();
        const mvpMat = m.create();
        const SURFACE_BACKGROUND = 0;
        const SURFACE_REFLECTION = 1;
        const SURFACE_REFRACTION = 2;
        let count = 0;
        function update() {
            clear();
            count++;
            const rad = ((count % 360) * Math.PI) / 180;
            const x = Math.cos(rad) * 3.5;
            const y = Math.sin(rad) * 3.5;
            const z = Math.sin(rad) * 3.5;
            q.toVecIII([0, 0, 20], qCamera, cameraPos);
            q.toVecIII([0, 1, 0], qCamera, cameraUp);
            m.lookAt(cameraPos, /* Camera center */ [0, 0, 0], cameraUp, vMat);
            m.multiply(pMat, vMat, vpMat);
            gl.uniform3fv(uniforms.eyePosition, cameraPos);
            // Render the cube object for background
            {
                bindObjectBuffers(cubeObject);
                m.identity(mMat);
                m.scale(mMat, [100, 100, 100], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniform1i(uniforms.surface, SURFACE_BACKGROUND);
                gl.drawElements(gl.TRIANGLES, cubeObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            const surface = reflectButton.checked ? SURFACE_REFLECTION : refractButton.checked ? SURFACE_REFRACTION : 3;
            gl.uniform1i(uniforms.surface, surface);
            // Render the torus object
            {
                bindObjectBuffers(torusObject);
                m.identity(mMat);
                m.translate(mMat, [x, -y, -z], mMat);
                m.rotate(mMat, -rad, /* axis */ [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                // Draw triangles based on the index buffer.
                gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            // Render the sphere object
            {
                bindObjectBuffers(sphereObject);
                m.identity(mMat);
                m.translate(mMat, [-x, y, z], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                // Draw triangles based on the index buffer.
                gl.drawElements(gl.TRIANGLES, sphereObject.lenIndices, 
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