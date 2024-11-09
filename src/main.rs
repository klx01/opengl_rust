mod shader;
mod mesh;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use shader::*;
use mesh::*;

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;

const VERT_SHADER_NOOP: &str = include_str!("shaders/noop.vert");
const VERT_SHADER_COLOUR: &str = include_str!("shaders/colour.vert");
const FRAG_SHADER_ORANGE: &str = include_str!("shaders/orange.frag");
const FRAG_SHADER_YELLOW: &str = include_str!("shaders/yellow.frag");
const FRAG_SHADER_COLOUR: &str = include_str!("shaders/input_colour.frag");

fn main() {
    let mut glfw = glfw::init(glfw::log_errors)
        .expect("Failed to init GLFW");

    glfw.window_hint(WindowHint::ContextVersionMajor(4));
    glfw.window_hint(WindowHint::ContextVersionMinor(1));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, _events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    /*// vertex shaders are guaranteed to have at least 16 vec4 inputs
    // you can check the actual amount using this
    // in my case it's still 16
    let mut res = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut res) };
    println!("max vertex attributes {res}");*/

    let (width, height) = window.get_framebuffer_size();
    window.set_framebuffer_size_callback(on_resize);
    on_resize(&mut window, width, height);

    /*let Some(program_orange) = ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_ORANGE) else {
        return;
    };
    let Some(program_yellow) = ShaderProgram::compile_vert_and_frag(VERT_SHADER_NOOP, FRAG_SHADER_YELLOW) else {
        return;
    };*/
    let Some(program_color) = ShaderProgram::compile_vert_and_frag(VERT_SHADER_COLOUR, FRAG_SHADER_COLOUR) else {
        return;
    };

    let mesh = rectangle_screen();
    //let meshes = two_triangles_old_split();
    //unsafe{gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)};

    while !window.should_close() {
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program_color.use_program();
        mesh.render();
        /*program_orange.use_program();
        meshes[0].render();
        program_yellow.use_program();
        meshes[1].render();*/

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn rectangle() -> Mesh {
    let vertices = [
         0.5,  0.5, 0.0,  // top right
         0.5, -0.5, 0.0,  // bottom right
        -0.5, -0.5, 0.0,  // bottom left
        -0.5,  0.5, 0.0   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,    // second triangle
    ];
    Mesh::new(&vertices, &indices)
}
fn rectangle_screen() -> Mesh {
    let vertices = [
         1.0,  1.0, 0.0,  // top right
         1.0, -1.0, 0.0,  // bottom right
        -1.0, -1.0, 0.0,  // bottom left
        -1.0,  1.0, 0.0   // top left
    ];
    let indices = [
        0, 1, 3,   // first triangle
        1, 2, 3,    // second triangle
    ];
    Mesh::new(&vertices, &indices)
}

fn old_triangle() -> OldMesh {
    let vertices = [
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
    ];
    OldMesh::new(&vertices)
}

fn two_triangles_old() -> OldMesh {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,

        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    OldMesh::new(&vertices)
}

fn two_triangles_old_split() -> [OldMesh; 2] {
    let vertices = [
        -1.0, -1.0, 0.0,
        -0.0, -1.0, 0.0,
        -0.5,  0.0, 0.0,
    ];
    let first = OldMesh::new(&vertices);
    let vertices = [
        0.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        0.5, 1.0, 0.0,
    ];
    let second = OldMesh::new(&vertices);
    [first, second]
}

fn on_resize(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width.into(), height.into()) };
}


