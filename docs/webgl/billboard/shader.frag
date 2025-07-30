precision mediump float;

uniform sampler2D texture;

varying vec2 vTextureCoord;

void main(void) {
    vec4 tex0 = texture2D(texture, vTextureCoord);
    if (tex0.w != 0.0) {
        gl_FragColor = tex0;
    } else {
        gl_FragColor = vec4(0.0);
    }
}
