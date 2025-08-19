precision mediump float;

uniform sampler2D texture;
uniform float alpha;

varying vec2 vTexCoord;

void main(void) {
    vec2 coord = vec2(vTexCoord.s, 1.0 - vTexCoord.t);
    gl_FragColor = vec4(texture2D(texture, coord).rgb, alpha);
}
