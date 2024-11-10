#version 410 core
layout (location = 0) in vec3 attr_position;
layout (location = 1) in vec3 attr_colour;
layout (location = 2) in vec2 attr_texture_coord;
out vec3 pass_colour;
out vec2 pass_texture_coord;

void main() {
    gl_Position = vec4(attr_position, 1.0);
    pass_colour = attr_colour;
    pass_texture_coord = attr_texture_coord;
}
