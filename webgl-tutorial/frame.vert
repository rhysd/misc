attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform mat4 mvpMat;
uniform mat4 mMat;

varying vec3 vModelPos;
varying vec4 vColor;
varying vec3 vNormal;
varying vec2 vTextureCoord;

void main(void) {
    vModelPos = (mMat * vec4(position, 1.0)).xyz;
    vColor = color;
    vNormal = normal;
    gl_Position = mvpMat * vec4(position, 1.0);
}
