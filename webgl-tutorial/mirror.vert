attribute vec3 position;
attribute vec2 texCoord;

uniform mat4 ortMat;

varying vec2 vTexCoord;

void main(void) {
    vTexCoord = texCoord;
    gl_Position = ortMat * vec4(position, 1.0);
}
