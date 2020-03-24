#version 140
in vec2 v_tex_coords;
in vec4 Color;
in vec4 Ambient;
out vec4 color;
uniform sampler2D tex;
void main() {
    color = texture(tex, v_tex_coords)*Color+Ambient*0.05;
}
