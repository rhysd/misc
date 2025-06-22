precision mediump float;

#define CANVAS_SIZE 512.0
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
    float tFrag = 1.0 / 512.0;
    vec2 fc = vec2(gl_FragCoord.s, CANVAS_SIZE - gl_FragCoord.t);
    vec3 horizontal = vec3(0.0);
    vec3 vertical = vec3(0.0);

    for (int j = 0; j <= 2; j++) {
        for (int i = 0; i <= 2; i++) {
            float x = float(i - 1);
            float y = float(j - 1);
            // An index value at array index access must be constant in WebGL/GLES2. However this
            // loop can be unrolled by a compiler and the index values are replaced with constants
            // so the following lines work. Note that using variable like `int idx = j * 3 + i;`
            // prevents the unrolling.
            horizontal += texture2D(texture, (fc + vec2(x, y)) * tFrag).rgb * sobelHorizontalKernel[j * 3 + i];
            vertical += texture2D(texture, (fc + vec2(x, y)) * tFrag).rgb * sobelVerticalKernel[j * 3 + i];
        }
    }

    return vec4(vec3(sqrt(horizontal * horizontal + vertical * vertical)), 1.0);
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
