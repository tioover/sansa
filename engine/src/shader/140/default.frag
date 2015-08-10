#version 140
uniform vec4 color_multiply;
uniform sampler2D tex;
in vec2 f_tex_coords;
out vec4 f_color;


void main() {
    f_color = texture(tex, f_tex_coords) * color_multiply;
}
