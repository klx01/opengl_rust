#version 410 core
in vec4 pass_colour;
out vec4 res_colour;

void main() {
    res_colour = pass_colour;
}