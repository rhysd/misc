"use strict";
(function () {
    const canvas = document.getElementById('canvas');
    canvas.width = 300;
    canvas.height = 300;
    const gl = canvas.getContext('webgl');
    const m = new matIV();
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
    function createObject(prog, positions, normals, colors, indices) {
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
    function createIndexBuffer(data) {
        const ibo = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, ibo);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Int16Array(data), gl.STATIC_DRAW);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
        return ibo;
    }
    function bindBuffers(object) {
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
        return [positions, normals, colors, indices];
    }
    function sphere(row, col, radius, color) {
        const positions = [];
        const normals = [];
        const colors = [];
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
                colors.push(...color);
            }
        }
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const r = (col + 1) * i + j;
                indices.push(r, r + 1, r + col + 2);
                indices.push(r, r + col + 2, r + col + 1);
            }
        }
        return [positions, normals, colors, indices];
    }
    async function main() {
        const [vs, fs] = await Promise.all([loadShader('shader.vert'), loadShader('shader.frag')]);
        gl.enable(gl.CULL_FACE);
        gl.enable(gl.DEPTH_TEST);
        gl.depthFunc(gl.LEQUAL);
        const prog = createProgram(vs, fs);
        const torusObject = createObject(prog, ...torus(64, 64, 0.75, 1.75));
        const sphereObject = createObject(prog, ...sphere(64, 64, 2, [0.25, 0.25, 0.75, 1]));
        const vMat = m.identity(m.create());
        const pMat = m.identity(m.create());
        const vpMat = m.identity(m.create());
        const eyeDirection = [0, 0, 20];
        m.lookAt(/* eye position */ eyeDirection, /* camera center */ [0, 0, 0], /* axis */ [0, 1, 0], vMat);
        m.perspective(
        /* fov */ 45, 
        /* aspect ratio */ canvas.width / canvas.height, 
        /* near clip */ 0.1, 
        /* far clip */ 100, pMat);
        m.multiply(pMat, vMat, vpMat);
        const uniforms = ['mvpMat', 'mMat', 'invMat', 'lightPosition', 'eyeDirection', 'ambientColor'].reduce((acc, name) => {
            acc[name] = gl.getUniformLocation(prog, name);
            return acc;
        }, {});
        const mMat = m.create();
        const mvpMat = m.create();
        const invMat = m.create();
        const lightPosition = [0, 0, 0];
        const ambientColor = [0.1, 0.1, 0.1, 1.0];
        let count = 0;
        function update() {
            clear();
            count++;
            const rad = ((count % 360) * Math.PI) / 180;
            const x = Math.cos(rad) * 3.5;
            const y = Math.sin(rad) * 3.5;
            const z = Math.sin(rad) * 3.5;
            gl.uniform3fv(uniforms.lightPosition, lightPosition);
            gl.uniform3fv(uniforms.eyeDirection, eyeDirection);
            gl.uniform4fv(uniforms.ambientColor, ambientColor);
            // Render torus object
            {
                bindBuffers(torusObject);
                m.identity(mMat);
                m.translate(mMat, [x, -y, -z], mMat);
                m.rotate(mMat, -rad, /* axis */ [0, 1, 1], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);
                // Draw triangles based on the index buffer.
                gl.drawElements(gl.TRIANGLES, torusObject.lenIndices, 
                /* type of index */ gl.UNSIGNED_SHORT, 
                /* start offset */ 0);
            }
            // Render sphere object
            {
                bindBuffers(sphereObject);
                m.identity(mMat);
                m.translate(mMat, [-x, y, z], mMat);
                m.multiply(vpMat, mMat, mvpMat);
                m.inverse(mMat, invMat);
                gl.uniformMatrix4fv(uniforms.mvpMat, /* transpose */ false, mvpMat);
                gl.uniformMatrix4fv(uniforms.mMat, /* transpose */ false, mMat);
                gl.uniformMatrix4fv(uniforms.invMat, /* transpose */ false, invMat);
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