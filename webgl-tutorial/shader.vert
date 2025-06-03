attribute vec3 position;
attribute vec4 color;
attribute vec3 normal;
attribute vec2 textureCoord;

uniform mat4 mvpMat;
uniform bool isOutline;

varying vec4 vColor;
varying vec2 vTextureCoord;

void main(void) {
    vColor = color;
    vTextureCoord = textureCoord;
    vec3 pos = position;
    if (isOutline) {
        pos += normal * 0.2; // The silhouette is 1.2x larger than the model
    }
    gl_Position = mvpMat * vec4(pos, 1.0);
}
