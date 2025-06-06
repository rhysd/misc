precision mediump float;

uniform sampler2D texture;
uniform float textureLength;
uniform float blur;

varying vec2 vTextureCoord;

void main(void) {
    vec4 color = texture2D(texture, vTextureCoord);

    if (blur > 0.0) {
        float denorm = blur / textureLength; // Division is expecive. Calculate it once.
        color *= 0.1;
        color += texture2D(texture, vTextureCoord + vec2(-1.0 * denorm,  1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2( 0.0 * denorm,  1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2( 1.0 * denorm,  1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2(-1.0 * denorm,  0.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2( 1.0 * denorm,  0.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2(-1.0 * denorm, -1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2( 0.0 * denorm, -1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2( 1.0 * denorm, -1.0 * denorm)) * 0.07;
        color += texture2D(texture, vTextureCoord + vec2(-2.0 * denorm,  2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-1.0 * denorm,  2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 0.0 * denorm,  2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 1.0 * denorm,  2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 2.0 * denorm,  2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-2.0 * denorm,  1.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 2.0 * denorm,  1.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-2.0 * denorm,  0.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 2.0 * denorm,  0.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-2.0 * denorm, -1.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 2.0 * denorm, -1.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-2.0 * denorm, -2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2(-1.0 * denorm, -2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 0.0 * denorm, -2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 1.0 * denorm, -2.0 * denorm)) * 0.03;
        color += texture2D(texture, vTextureCoord + vec2( 2.0 * denorm, -2.0 * denorm)) * 0.03;
    }

    gl_FragColor = color;
}
