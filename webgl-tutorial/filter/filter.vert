attribute vec3 position;
attribute vec2 texCoord;

uniform mat4 mvpMat;

varying vec2 vTexCoord;

void main(void){
    vTexCoord = texCoord;
    gl_Position = mvpMat * vec4(position, 1.0);
}
