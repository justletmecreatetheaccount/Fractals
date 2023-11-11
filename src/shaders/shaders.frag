#version 410 core
#ifndef GL_ES
precision highp float;
#endif

uniform int fractal_type;
uniform int MAX_ITERATIONS;
uniform float fractal_precision;
uniform vec2 iResolution;
uniform float theta;
uniform vec2 offset;
uniform float zoom;


out vec4 color;

//square complex number 
// (a+ib)^2 = a^2 + 2iab - b^2

vec4 debug (vec2 coord) {
    return vec4(coord.x, coord.y, 0.0, 1.0);
}

vec4 process_julia(vec2 coord) {
    int loop_count = 0;
    vec2 prev_complex = vec2(0, 0);
    vec2 current_complex = coord;
    while (length(prev_complex - current_complex) < fractal_precision && loop_count < MAX_ITERATIONS) {
        prev_complex = current_complex;
        current_complex = vec2(prev_complex.x * prev_complex.x - prev_complex.y * prev_complex.y + cos(theta), 
        2.0 * prev_complex.x * prev_complex.y + sin(theta));
        loop_count++;
    }   
    float count = float(loop_count);
    float FMAX_ITERATIONS = float(MAX_ITERATIONS);
    return vec4(1.0 - count/FMAX_ITERATIONS, 1.0 - count/FMAX_ITERATIONS, 1.0 - count/FMAX_ITERATIONS, 1.0);
}

void main() {
    vec2 trueCoord = (gl_FragCoord.xy / iResolution.xy * zoom + offset) * 2.0 - 1.0 * zoom;
    if (fractal_type == 0) {
        color = debug(trueCoord);
    } else if (fractal_type == 1) {
        color = process_julia(trueCoord);
    } else {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
}