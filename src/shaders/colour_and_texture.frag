#version 410 core
in vec3 pass_colour;
in vec2 pass_texture_coord;
out vec4 res_colour;
uniform sampler2D set_texture0;
uniform sampler2D set_texture1;
uniform float interpolation;

void main() {
    res_colour = mix(texture(set_texture0, pass_texture_coord), texture(set_texture1, pass_texture_coord), interpolation)
        //* vec4(pass_colour, 1.0)
    ;
}