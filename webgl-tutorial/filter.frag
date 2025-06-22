precision mediump float;

#define FILTER_GRAYSCALE 1
#define FILTER_SOBEL 2

uniform sampler2D texture;
uniform int filter;
uniform float sobelHorizontalKernel[9];
uniform float sobelVerticalKernel[9];

varying vec2 vTexCoord;

vec4 grayScaleFilter() {
    const float redScale = 0.298912;
    const float greenScale = 0.586611;
    const float blueScale = 0.114478;
    const vec3 monochromeScale = vec3(redScale, greenScale, blueScale);
    vec4 sampled = texture2D(texture, vTexCoord);
    float gray = dot(sampled.rgb, monochromeScale);
    return vec4(vec3(gray), 1.0);
}

vec4 sobelFilter() {
    vec2 offset[9];
    offset[0] = vec2(-1.0, -1.0);
    offset[1] = vec2( 0.0, -1.0);
    offset[2] = vec2( 1.0, -1.0);
    offset[3] = vec2(-1.0,  0.0);
    offset[4] = vec2( 0.0,  0.0);
    offset[5] = vec2( 1.0,  0.0);
    offset[6] = vec2(-1.0,  1.0);
    offset[7] = vec2( 0.0,  1.0);
    offset[8] = vec2( 1.0,  1.0);
    float tFrag = 1.0 / 512.0;
    vec2  fc = vec2(gl_FragCoord.s, 512.0 - gl_FragCoord.t);
    vec3  horizonColor = vec3(0.0);
    vec3  verticalColor = vec3(0.0);
    vec4  destColor = vec4(0.0);

    horizonColor  += texture2D(texture, (fc + offset[0]) * tFrag).rgb * sobelHorizontalKernel[0];
    horizonColor  += texture2D(texture, (fc + offset[1]) * tFrag).rgb * sobelHorizontalKernel[1];
    horizonColor  += texture2D(texture, (fc + offset[2]) * tFrag).rgb * sobelHorizontalKernel[2];
    horizonColor  += texture2D(texture, (fc + offset[3]) * tFrag).rgb * sobelHorizontalKernel[3];
    horizonColor  += texture2D(texture, (fc + offset[4]) * tFrag).rgb * sobelHorizontalKernel[4];
    horizonColor  += texture2D(texture, (fc + offset[5]) * tFrag).rgb * sobelHorizontalKernel[5];
    horizonColor  += texture2D(texture, (fc + offset[6]) * tFrag).rgb * sobelHorizontalKernel[6];
    horizonColor  += texture2D(texture, (fc + offset[7]) * tFrag).rgb * sobelHorizontalKernel[7];
    horizonColor  += texture2D(texture, (fc + offset[8]) * tFrag).rgb * sobelHorizontalKernel[8];

    verticalColor += texture2D(texture, (fc + offset[0]) * tFrag).rgb * sobelVerticalKernel[0];
    verticalColor += texture2D(texture, (fc + offset[1]) * tFrag).rgb * sobelVerticalKernel[1];
    verticalColor += texture2D(texture, (fc + offset[2]) * tFrag).rgb * sobelVerticalKernel[2];
    verticalColor += texture2D(texture, (fc + offset[3]) * tFrag).rgb * sobelVerticalKernel[3];
    verticalColor += texture2D(texture, (fc + offset[4]) * tFrag).rgb * sobelVerticalKernel[4];
    verticalColor += texture2D(texture, (fc + offset[5]) * tFrag).rgb * sobelVerticalKernel[5];
    verticalColor += texture2D(texture, (fc + offset[6]) * tFrag).rgb * sobelVerticalKernel[6];
    verticalColor += texture2D(texture, (fc + offset[7]) * tFrag).rgb * sobelVerticalKernel[7];
    verticalColor += texture2D(texture, (fc + offset[8]) * tFrag).rgb * sobelVerticalKernel[8];

    return vec4(vec3(sqrt(horizonColor * horizonColor + verticalColor * verticalColor)), 1.0);
}

void main(void){
    if (filter == FILTER_GRAYSCALE) {
        gl_FragColor = grayScaleFilter();
    } else if (filter == FILTER_SOBEL) {
        gl_FragColor = sobelFilter();
    } else {
        gl_FragColor = texture2D(texture, vTexCoord);
    }
}
