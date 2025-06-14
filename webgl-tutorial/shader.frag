precision mediump float;

uniform mat4 invMat;
uniform vec3 lightPosition;
uniform sampler2D texture;

varying vec4 vColor;
varying vec3 vNormal;
varying vec3 vPosition;
varying vec4 vTexCoord;

void main(void) {
    vec3 light = lightPosition - vPosition;
    vec3 invLight = normalize(invMat * vec4(light, 0.0)).xyz;
    float diffuse = clamp(dot(vNormal, invLight), 0.1, 1.0);
    vec4 sampled = texture2DProj(texture, vTexCoord);
    gl_FragColor = vColor * vec4(vec3(diffuse), 1.0) * sampled;
}
