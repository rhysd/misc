precision mediump float;

uniform mat4 invMat;
uniform vec3 lightDirection;
uniform vec3 eyeDirection;
uniform vec4 ambientColor;

varying vec4 vColor;
varying vec3 vNormal;

void main(void) {
    vec3 invLight = normalize(invMat * vec4(lightDirection, 1.0)).xyz;
    vec3 invEye = normalize(invMat * vec4(eyeDirection, 1.0)).xyz;
    vec3 invHalf = normalize(invLight + invEye);
    float specular = pow(clamp(dot(vNormal, invHalf), 0.0, 1.0), 50.0);
    float diffuse = clamp(dot(vNormal, invLight), 0.0, 1.0);
    vec4 lightColor = vColor * vec4(vec3(diffuse + specular), 1.0);
    gl_FragColor = lightColor + ambientColor;
}
