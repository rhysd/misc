attribute vec3 position;
attribute vec3 normal;

uniform mat4 mvpMat;
uniform mat4 mMat;

varying vec3 vModelPos;
varying vec3 vNormal;

void main(void) {
    vModelPos = (mMat * vec4(position, 1.0)).xyz;
    vNormal = (mMat * vec4(normal, 0.0)).xyz;
    gl_Position = mvpMat * vec4(position, 1.0);
}
