#version 410 core
layout (location = 0) in vec3 position;
out vec4 vert_colour;

void main() {
    gl_Position = vec4(position, 1.0);
    vert_colour = vec4((position.xy + 1.0) / 2, position.z, 1.0);
}
