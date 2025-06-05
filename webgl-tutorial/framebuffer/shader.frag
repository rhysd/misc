precision mediump float;

uniform mat4 invMat;
uniform vec3 lightPosition;
uniform vec3 eyeDirection;
uniform vec4 ambientColor;
uniform sampler2D texture;
uniform bool useTexture;

varying vec3 vPosition;
varying vec4 vColor;
varying vec3 vNormal;
varying vec2 vTextureCoord;

void main(void) {
    if (useTexture) {
        gl_FragColor = texture2D(texture, vTextureCoord);
        return;
    }

    vec3 light = lightPosition - vPosition;
    vec3 invLight = normalize(invMat * vec4(light, 0.0)).xyz;
    vec3 invEye = normalize(invMat * vec4(eyeDirection, 0.0)).xyz;
    vec3 invHalf = normalize(invLight + invEye);
    float diffuse = clamp(dot(vNormal, invLight), 0.0, 1.0) + 0.2;
    float specular = pow(clamp(dot(vNormal, invHalf), 0.0, 1.0), 50.0);
    vec4 lightColor = vColor * vec4(vec3(diffuse + specular), 1.0);
    gl_FragColor = lightColor + ambientColor;
}
