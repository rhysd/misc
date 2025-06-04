precision mediump float;

uniform sampler2D texture; // Texture for billboard
uniform bool useTexture;

varying vec4 vColor;
varying vec2 vTextureCoord;

void main(void) {
    if (useTexture) {
        gl_FragColor = vColor * texture2D(texture, vTextureCoord);
    } else {
        gl_FragColor = vColor;
    }
}
