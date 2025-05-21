precision mediump float;

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform int useTexture;

varying vec4 vColor;
varying vec2 vTextureCoord;

void main(void) {
    if (!bool(useTexture)) {
        gl_FragColor = vColor;
        return;
    }

    // The texture0 is over the texture1.

    vec4 tex0 = texture2D(texture0, vTextureCoord);
    if (tex0.w != 0.0) {
        gl_FragColor = tex0;
        return;
    }

    vec4 tex1 = texture2D(texture1, vTextureCoord);
    if (tex1.w != 0.0) {
        gl_FragColor = tex1;
        return;
    }

    gl_FragColor = vec4(1.0); // Use white background for textures
}
