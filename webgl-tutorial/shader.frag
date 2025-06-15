precision mediump float;

uniform mat4 invMat;
uniform vec3 lightPosition;
uniform sampler2D texture;

varying vec4 vColor;
varying vec3 vNormal;
varying vec3 vPosition;
varying vec4 vTexCoord;
varying vec4 vDepth;

float decodeDepthFromRGBA(vec4 rgba) {
    float rMask = 1.0;
    float gMask = 1.0 / 255.0;
    float bMask = 1.0 / (255.0 * 255.0);
    float aMask = 1.0 / (255.0 * 255.0 * 255.0);
    float depth = dot(rgba, vec4(rMask, gMask, bMask, aMask));
    return depth;
}

void main(void) {
    vec3 light = lightPosition - vPosition;
    vec3 invLight = normalize(invMat * vec4(light, 0.0)).xyz;
    float diffuse = clamp(dot(vNormal, invLight), 0.2, 1.0);

    vec4 depthColor = vec4(1.0);
    if (vDepth.w > 0.0) {
        float depth = decodeDepthFromRGBA(texture2DProj(texture, vTexCoord));
        vec4 lightCoord = vDepth / vDepth.w;
        // Subtract 0.0001 to avoid Mach bands when `lightCoord.z` is equal to `depth`
        if (lightCoord.z - 0.0001 > depth) {
            // When the fragment is in shadow
            depthColor = vec4(0.5, 0.5, 0.5, 1.0);
        }
    }
    gl_FragColor = vColor * vec4(vec3(diffuse), 1.0) * depthColor;
}
