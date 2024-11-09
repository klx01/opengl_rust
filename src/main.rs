mod shader;
mod mesh;
mod meshes;
mod shaders;

use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use shader::*;
use shaders::*;
use mesh::*;
use meshes::*;

const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;

fn main() -> Result<(), ()> {
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

    //let program_orange = program_orange().ok_or(())?;
    //let program = program_upside_down().ok_or(())?;
    let program = program_offset().ok_or(())?;
    //let program_yellow = program_yellow().ok_or(())?;
    //let program = program_pos_colour().ok_or(())?;
    //let program = program_set_colour().ok_or(())?;
    //let program = program_in_colour().ok_or(())?;

    let (width, height) = window.get_framebuffer_size();
    window.set_framebuffer_size_callback(on_resize);
    on_resize(&mut window, width, height);

    //let mesh = rectangle_screen();
    let mesh = multi_attr();
    //let meshes = two_triangles_old_split();
    //unsafe{gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)};

    while !window.should_close() {
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        } 
        let time = glfw.get_time();
        let green = (time.sin() / 2.0) + 0.5;
        let offset_x = time.sin() / 2.0;
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        program.use_program();
        // to set the uniform value, you need to use the program first
        //program.set_location(0, 0.0, green as f32, 0.0, 1.0);
        program.set_location(0, offset_x as f32, 0.0, 0.0, 0.0);
        mesh.render();
        /*program_orange.use_program();
        meshes[0].render();
        program_yellow.use_program();
        meshes[1].render();*/

        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

fn on_resize(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width.into(), height.into()) };
}


