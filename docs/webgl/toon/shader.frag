precision mediump float;

#define MAX_TOON_GRADATION 10

uniform mat4 invMat;
uniform vec3 lightDirection;
uniform bool isOutline;
uniform float toonThresholds[MAX_TOON_GRADATION];
uniform float toonGradation[MAX_TOON_GRADATION];

varying vec4 vColor;
varying vec3 vNormal;

void main(void) {
    if (isOutline) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    vec3 invLight = normalize(invMat * vec4(lightDirection, 0.0)).xyz;
    float diffuse = clamp(dot(vNormal, invLight), 0.00, 1.0);

    vec4 light = vec4(0.0);
    for (int i = 0; i < MAX_TOON_GRADATION; i++) {
        float threshold = toonThresholds[i];
        if (diffuse <= threshold) {
            light = vec4(vec3(toonGradation[i]), 1.0);
            break;
        }
    }
    gl_FragColor = vColor * light;
}
