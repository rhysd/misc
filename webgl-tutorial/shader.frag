precision mediump float;

uniform mat4 invMat;
uniform vec3 lightDirection;
uniform bool isOutline;
uniform sampler2D toonTexture;

varying vec4 vColor;
varying vec3 vNormal;

void main(void) {
    if (isOutline) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }
    vec3 invLight = normalize(invMat * vec4(lightDirection, 0.0)).xyz;
    float diffuse = clamp(dot(vNormal, invLight), 0.00, 1.0);
    vec4 light = texture2D(toonTexture, vec2(diffuse, 0.0));
    gl_FragColor = vColor * light;
}
