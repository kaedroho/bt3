#version 120

uniform sampler2D t_Heightmap;

varying vec2 v_Uv;

void main() {
    vec3 height = vec3(texture2D(t_Heightmap, v_Uv).r);
    gl_FragColor = vec4(height, 1.0);
}
