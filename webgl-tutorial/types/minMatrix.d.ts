type Vec3 = [number, number, number];

declare class matIV {
    create(): Float32Array;
    identity(dest: Float32Array): Float32Array;
    multiply(m1: Float32Array, m2: Float32Array, dest: Float32Array): Float32Array;
    scale(m: Float32Array, vec: Vec3, dest: Float32Array): Float32Array;
    translate(m: Float32Array, vec: Vec3, dest: Float32Array): Float32Array;
    rotate(m: Float32Array, angle: number, axis: Vec3, dest: Float32Array): Float32Array;
    lookAt(eye: Vec3, center: Vec3, axis: Vec3, dest: Float32Array): Float32Array;
    perspective(fovy: number, aspect: number, near: number, far: number, dest: Float32Array): Float32Array;
    ortho(
        left: number,
        right: number,
        top: number,
        bottom: number,
        near: number,
        far: number,
        dest: Float32Array,
    ): Float32Array;
    transpose(m: Float32Array, dest: Float32Array): Float32Array;
    inverse(m: Float32Array, dest: Float32Array): Float32Array;
}

declare class qtnIV {
    create(): Float32Array;
    identity(dest: Float32Array): Float32Array;
    inverse(q: Float32Array, dest: Float32Array): Float32Array;
    normalize(dest: Float32Array): Float32Array;
    multiply(q1: Float32Array, q2: Float32Array, dest: Float32Array): Float32Array;
    rotate(angle: number, axis: Vec3, dest: Float32Array): Float32Array;
    toVecIII(vec: Vec3, qtn: Float32Array, dest: Vec3): Vec3;
    toMatIV(q: Float32Array, dest: Float32Array): Float32Array;
    slerp(qtn1: Float32Array, qtn2: Float32Array, time: number, dest: Float32Array): Float32Array;
}
