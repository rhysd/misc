precision mediump float;

uniform sampler2D texture; // Texture for billboard
uniform int useTexture;

varying vec4 vColor;

void main(void) {
    if (bool(useTexture)) {
        gl_FragColor = texture2D(texture, gl_PointCoord);
    } else {
        gl_FragColor = vColor;
    }
}
