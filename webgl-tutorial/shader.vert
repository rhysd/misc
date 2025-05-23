attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform mat4 mvpMat;

varying vec4 vColor;
varying vec3 vNormal;

void main(void) {
    vColor = color;
    vNormal = normal;
    gl_Position = mvpMat * vec4(position, 1.0);
}
