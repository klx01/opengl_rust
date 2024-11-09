#version 410 core
uniform vec4 set_colour;
out vec4 res_colour;

void main() {
    res_colour = set_colour;
}