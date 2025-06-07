attribute vec3 position;
attribute vec2 textureCoord;

uniform mat4 mvpMat;

varying vec2 vTextureCoord;

void main(void) {
    vTextureCoord = textureCoord;
    gl_Position = mvpMat * vec4(position, 1.0);
}
