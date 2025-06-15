attribute vec3 position;

uniform mat4 mvpMat;

void main(void) {
    gl_Position = mvpMat * vec4(position, 1.0);
}
