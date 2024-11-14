use crate::shader::*;

const VERT_SHADER_NOOP: &str = include_str!("shaders/noop.vert");
const VERT_SHADER_UPSIDE_DOWN: &str = include_str!("shaders/upside_down.vert");
const VERT_SHADER_OFFSET: &str = include_str!("shaders/offset.vert");
const VERT_SHADER_POS_COLOUR: &str = include_str!("shaders/position_colour.vert");
const VERT_SHADER_IN_COLOUR: &str = include_str!("shaders/input_colour.vert");
const VERT_SHADER_COLOUR_AND_TEXTURE: &str = include_str!("shaders/colour_and_texture.vert");
const VERT_SHADER_TRANSFORM: &str = include_str!("shaders/transform.vert");
const VERT_SHADER_MVP: &str = include_str!("shaders/mvp.vert");
const VERT_SHADER_MVP_TEXTURE: &str = include_str!("shaders/mvp_texture.vert");
const FRAG_SHADER_ORANGE: &str = include_str!("shaders/orange.frag");
const FRAG_SHADER_YELLOW: &str = include_str!("shaders/yellow.frag");
const FRAG_SHADER_IN_COLOUR: &str = include_str!("shaders/input_colour.frag");
const FRAG_SHADER_UNI_COLOUR: &str = include_str!("shaders/uniform_colour.frag");
const FRAG_SHADER_COLOUR_AND_TEXTURE: &str = include_str!("shaders/colour_and_texture.frag");
const FRAG_SHADER_TEXTURE: &str = include_str!("shaders/texture.frag");

pub(crate) fn program_orange() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_ORANGE)
}

pub(crate) fn program_upside_down() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_UPSIDE_DOWN, FRAG_SHADER_ORANGE)
}

pub(crate) fn program_offset() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_OFFSET, FRAG_SHADER_ORANGE)?;
    ProgramWithUniforms::new(program, &[c"offset"])
}

pub(crate) fn program_yellow() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_YELLOW)
}

pub(crate) fn program_pos_colour() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_POS_COLOUR, FRAG_SHADER_IN_COLOUR)
}

pub(crate) fn program_transform() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_TRANSFORM, FRAG_SHADER_IN_COLOUR)?;
    ProgramWithUniforms::new(program, &[c"set_transform"])
}

pub(crate) fn program_mvp() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_MVP, FRAG_SHADER_IN_COLOUR)?;
    ProgramWithUniforms::new(program, &[c"set_model", c"set_view", c"set_projection"])
}

pub(crate) fn program_mvp_texture() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_MVP_TEXTURE, FRAG_SHADER_TEXTURE)?;
    ProgramWithUniforms::new(program, &[c"set_model", c"set_view", c"set_projection", c"set_texture0", c"set_texture1"])
}

pub(crate) fn program_in_colour() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_IN_COLOUR, FRAG_SHADER_IN_COLOUR)
}

pub(crate) fn program_colour_and_texture() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_COLOUR_AND_TEXTURE, FRAG_SHADER_COLOUR_AND_TEXTURE)?;
    ProgramWithUniforms::new(program, &[c"set_texture0", c"set_texture1", c"interpolation"])
}

pub(crate) fn program_set_colour() -> Option<ProgramWithUniforms> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_UNI_COLOUR)?;
    ProgramWithUniforms::new(program, &[c"set_colour"])
}
