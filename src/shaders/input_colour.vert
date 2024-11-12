#version 410 core
layout (location = 0) in vec3 attr_position;
layout (location = 1) in vec3 attr_colour;
out vec4 pass_colour;

void main() {
    gl_Position = vec4(attr_position, 1.0);
    pass_colour = vec4(attr_colour, 1.0);
}
