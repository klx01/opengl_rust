#version 410 core
layout (location = 0) in vec3 attr_position;
out vec4 pass_colour;
uniform mat4 set_model;
uniform mat4 set_view;
uniform mat4 set_projection;

void main() {
    gl_Position = set_projection * set_view * set_model * vec4(attr_position, 1.0);
    pass_colour = vec4((attr_position.xy + 1.0) / 2, attr_position.z, 1.0);
}
