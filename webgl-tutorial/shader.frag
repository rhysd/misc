precision mediump float;

uniform sampler2D texture;

varying vec4 vColor;
varying vec2 vTextureCoord;

void main(void) {
    vec4 texColor = texture2D(texture, vTextureCoord);
    gl_FragColor = vColor * texColor;
}
