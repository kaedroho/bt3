#version 150 core

in vec3 a_Pos;
out vec3 v_Color;

uniform sampler2D t_Heightmap;
uniform mat4 u_MVP;
uniform vec2 u_Offset;

void main() {
    vec2 pos = vec2(u_Offset) + vec2(a_Pos) * 16;

    float height = texture(t_Heightmap, pos/2048).r;

    v_Color = vec3(height, 1-height, 0);
    gl_Position = u_MVP * vec4(pos / 16, height * 50, 1.0);
}
