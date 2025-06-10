precision mediump float;

uniform vec3 eyePosition;
uniform samplerCube envTexture;
uniform int surface;

varying vec3 vModelPos;
varying vec3 vNormal;

#define BACKGROUND 0
#define REFLECTION 1
#define REFRACTION 2

void main(void) {
    vec3 coord;
    if (surface == BACKGROUND) {
        coord = vNormal;
    } else if (surface == REFLECTION) {
        coord = reflect(vModelPos - eyePosition, vNormal);
    } else if (surface == REFRACTION) {
        float refractiveIndex = 0.6;
        coord = refract(normalize(vModelPos - eyePosition), vNormal, refractiveIndex);
    } else {
        discard;
        return;
    }
    gl_FragColor = textureCube(envTexture, coord);
}
