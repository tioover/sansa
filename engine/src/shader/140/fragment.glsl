#version 140
uniform vec4 color_multiply;
uniform vec2 tex_offset;
uniform sampler2D tex;
in vec2 f_tex_coords;
out vec4 f_color;

vec2 query_size(sampler2D s) {
    return textureSize(s, 0);
}

vec2 coords_covert(vec2 size, vec2 coords) {
    coords = coords / size;
    coords.y = 1 - coords.y;
    return coords;
}

void main() {
    vec2 size = query_size(tex);
    vec2 coords = f_tex_coords + tex_offset;
    f_color = texture(tex, coords_covert(size, coords)) * color_multiply;
}
