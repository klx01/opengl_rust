use crate::shader::{IProgram, ShaderProgram};

const VERT_SHADER_NOOP: &str = include_str!("shaders/noop.vert");
const VERT_SHADER_POS_COLOUR: &str = include_str!("shaders/position_colour.vert");
const VERT_SHADER_IN_COLOUR: &str = include_str!("shaders/input_colour.vert");
const FRAG_SHADER_ORANGE: &str = include_str!("shaders/orange.frag");
const FRAG_SHADER_YELLOW: &str = include_str!("shaders/yellow.frag");
const FRAG_SHADER_IN_COLOUR: &str = include_str!("shaders/input_colour.frag");
const FRAG_SHADER_UNI_COLOUR: &str = include_str!("shaders/uniform_colour.frag");

pub(crate) fn program_orange() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_ORANGE)
}

pub(crate) fn program_yellow() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_YELLOW)
}

pub(crate) fn program_pos_colour() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_POS_COLOUR, FRAG_SHADER_IN_COLOUR)
}

pub(crate) fn program_in_colour() -> Option<ShaderProgram> {
    ShaderProgram::compile_vert_and_frag(VERT_SHADER_IN_COLOUR, FRAG_SHADER_IN_COLOUR)
}

pub(crate) fn program_set_colour() -> Option<ProgramSetColour> {
    let program = ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_UNI_COLOUR)?;
    ProgramSetColour::new(program)
}

pub(crate) struct ProgramSetColour {
    inner: ShaderProgram,
    set_colour_location: i32,
}
impl ProgramSetColour {
    pub(crate) fn new(inner: ShaderProgram) -> Option<Self> {
        let set_colour_location = inner.get_uniform_location(c"set_colour");
        if set_colour_location < 0 {
            return None;
        }
        Some(Self { inner, set_colour_location })
    }
    pub(crate) fn inner(&self) -> &ShaderProgram {
        &self.inner
    }
    pub(crate) fn get_set_colour_location(&self) -> i32 {
        self.set_colour_location
    }
    pub(crate) fn set_colour(&self, r: f32, g: f32, b: f32, a: f32) {
        self.inner.set_uniform(self.set_colour_location, r, g, b, a)
    }
}
impl IProgram for ProgramSetColour {
    fn use_program(&self) {
        self.inner.use_program();
    }
}
