#version 150 core

uniform sampler2D t_Heightmap;

in vec2 v_Uv;
out vec4 o_Color;

void main() {
    vec3 height = vec3(texture(t_Heightmap, v_Uv).r);
    o_Color = vec4(height, 1.0);
}
