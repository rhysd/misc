attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform mat4 mvpMat;
uniform bool isOutline;

varying vec4 vColor;
varying vec3 vNormal;

void main(void) {
    vColor = color;
    vNormal = normal;
    vec3 pos = position;
    if (isOutline) {
        pos += normal * 0.05;
    }
    gl_Position = mvpMat * vec4(pos, 1.0);
}
