attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform bool isShadow;
uniform mat4 mMat;
uniform mat4 tMat; // Texture transformation matrix
uniform mat4 mvpMat;
uniform mat4 lgtMat; // Coordnate conversion matrix for light

varying vec4 vColor;
varying vec3 vNormal;
varying vec3 vPosition;
varying vec4 vTexCoord;
varying vec4 vDepth;

void main(void) {
    if (!isShadow) {
        vColor = color;
        vNormal = normal;
        vPosition = (mMat * vec4(position, 1.0)).xyz;
        vTexCoord = tMat * vec4(vPosition, 1.0); // Transform texture coordinates
        vDepth = lgtMat * vec4(position, 1.0);
    }
    gl_Position = mvpMat * vec4(position, 1.0);
}
