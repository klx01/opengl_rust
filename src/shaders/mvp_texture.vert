#version 410 core
layout (location = 0) in vec3 attr_position;
layout (location = 1) in vec2 attr_texture_coord;
out vec2 pass_texture_coord;
uniform mat4 set_model;
uniform mat4 set_view;
uniform mat4 set_projection;

void main() {
    gl_Position = set_projection * set_view * set_model * vec4(attr_position, 1.0);
    pass_texture_coord = attr_texture_coord;
}
