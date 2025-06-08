precision mediump float;

uniform vec3 eyePosition;
uniform samplerCube envTexture;
uniform bool isBackground;

varying vec3 vModelPos;
varying vec3 vNormal;

void main(void) {
    vec3 coord;
    if (isBackground) {
        coord = vNormal;
    } else {
        coord = reflect(vModelPos - eyePosition, vNormal);
    }
    gl_FragColor = textureCube(envTexture, coord);
}
