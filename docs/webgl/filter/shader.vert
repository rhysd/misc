attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform mat4 mvpMat;
uniform mat4 invMat;
uniform vec3 lightDirection;
uniform vec3 eyePosition;
uniform vec4 ambientColor;

varying   vec4 vColor;

void main(void){
    vec3 invLight = normalize(invMat * vec4(lightDirection, 0.0)).xyz;
    vec3 invEye = normalize(invMat * vec4(eyePosition, 0.0)).xyz;
    vec3 halfLE = normalize(invLight + invEye);
    float diffuse = clamp(dot(normal, invLight), 0.0, 1.0);
    float specular = pow(clamp(dot(normal, halfLE), 0.0, 1.0), 50.0);
    vColor = color * ambientColor * vec4(vec3(diffuse), 1.0) + vec4(vec3(specular), 1.0);
    gl_Position = mvpMat * vec4(position, 1.0);
}
