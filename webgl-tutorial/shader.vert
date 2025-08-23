attribute vec3 position;
attribute vec3 normal;
attribute vec4 color;

uniform mat4 mvpMat;
uniform mat4 invMat;
uniform vec3 lightDirection;
uniform vec3 skyDirection;
uniform vec4 skyColor;
uniform vec4 groundColor;
uniform vec3 eyePosition;
uniform bool isGround;

varying vec4 vColor;

void main(void) {
    gl_Position = mvpMat * vec4(position, 1.0);

    if (isGround) {
        vColor = color;
        return;
    }

    vec3 invSky = normalize(invMat * vec4(skyDirection, 0.0)).xyz;
    vec3 invLight = normalize(invMat * vec4(lightDirection, 0.0)).xyz;
    vec3 invEye = normalize(invMat * vec4(eyePosition, 0.0)).xyz;
    vec3 halfLightEye = normalize(invLight + invEye);
    float diffuse = clamp(dot(normal, invLight), 0.1, 1.0);
    float specular = pow(clamp(dot(normal, halfLightEye), 0.0, 1.0), 50.0);
    // `dot` calculates cos. `+ 1.0` and `* 0.5` normalizes the result from [-1, 1] to [0, 1]
    float hemisphere = (dot(normal, invSky) + 1.0) * 0.5;
    vec4 ambient = mix(groundColor, skyColor, hemisphere);
    vColor = color * vec4(vec3(diffuse), 1.0) + vec4(vec3(specular), 1.0) + ambient;
}
