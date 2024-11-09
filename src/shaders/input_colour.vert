#version 410 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 colour;
out vec4 vert_colour;

void main() {
    gl_Position = vec4(position, 1.0);
    vert_colour = vec4(colour, 1.0);
}
