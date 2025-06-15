precision mediump float;

// Store 32bit float value in vec4 instance
vec4 encodeDepthToRGBA(float depth) {
    float r = depth;
    float g = fract(r * 255.0);
    float b = fract(g * 255.0);
    float a = fract(b * 255.0);
    float co = 1.0 / 255.0;
    r -= g * co;
    g -= b * co;
    b -= a * co;
    return vec4(r, g, b, a);
}

void main(void) {
    // `gl_FragCoord.z` stores the value of the depth buffer of the fragment
    gl_FragColor = encodeDepthToRGBA(gl_FragCoord.z);
}
