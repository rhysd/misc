precision mediump float;

#define CANVAS_SIZE 512.0
#define FILTER_GRAYSCALE 1
#define FILTER_SOBEL 2
#define FILTER_LAPLACIAN 3
#define FILTER_GAUSSIAN 4

uniform sampler2D texture;
uniform int filter;
uniform float canvasHeight;
uniform float filterKernel[9];
uniform float gaussianWeight[10];
uniform bool gaussianIsHorizontal;

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
    float norm = 1.0 / canvasHeight;
    vec2 origin = vec2(gl_FragCoord.s, canvasHeight - gl_FragCoord.t);
    vec3 horizontal = vec3(0.0);
    vec3 vertical = vec3(0.0);

    for (int j = 0; j <= 2; j++) {
        for (int i = 0; i <= 2; i++) {
            float x = float(i - 1);
            float y = float(j - 1);
            // An index value at array index access must be constant in WebGL/GLES2. However this
            // loop can be unrolled by a compiler and the index values are replaced with constants
            // so the following line work.
            float coefficient = filterKernel[j * 3 + i];
            horizontal += texture2D(texture, (origin + vec2(x, y)) * norm).rgb * coefficient;
            vertical += texture2D(texture, (origin + vec2(y, x)) * norm).rgb * coefficient;
        }
    }

    return vec4(vec3(sqrt(horizontal * horizontal + vertical * vertical)), 1.0);
}

vec4 laplacianFilter() {
    float norm = 1.0 / canvasHeight;
    vec2 origin = vec2(gl_FragCoord.s, canvasHeight - gl_FragCoord.t);
    vec3 color = vec3(0.0);

    for (int j = 0; j <= 2; j++) {
        for (int i = 0; i <= 2; i++) {
            float x = float(i - 1);
            float y = float(j - 1);
            vec2 pos = (origin + vec2(x, y)) * norm;
            color += texture2D(texture, pos).rgb * filterKernel[j * 3 + i];
        }
    }

    return vec4(color, 1.0);
}

vec4 gaussianFilter() {
    float norm = 1.0 / canvasHeight;
    vec3 color = vec3(0.0);

    vec2 origin = vec2(gl_FragCoord.s, canvasHeight - gl_FragCoord.t);
    if (gaussianIsHorizontal) {
        for (int i = -9; i <= 9; i++) {
            vec2 pos = origin + vec2(float(i), 0.0);
            float weight = gaussianWeight[i >= 0 ? i : -i];
            color += texture2D(texture, pos * norm).rgb * weight;
        }
    } else {
        for (int i = -9; i <= 9; i++) {
            vec2 pos = origin + vec2(0.0, float(i));
            float weight = gaussianWeight[i >= 0 ? i : -i];
            color += texture2D(texture, pos * norm).rgb * weight;
        }
    }

    return vec4(color, 1.0);
}

void main(void){
    if (filter == FILTER_GRAYSCALE) {
        gl_FragColor = grayScaleFilter();
    } else if (filter == FILTER_SOBEL) {
        gl_FragColor = sobelFilter();
    } else if (filter == FILTER_LAPLACIAN) {
        gl_FragColor = laplacianFilter();
    } else if (filter == FILTER_GAUSSIAN) {
        gl_FragColor = gaussianFilter();
    } else {
        gl_FragColor = texture2D(texture, vTexCoord);
    }
}
