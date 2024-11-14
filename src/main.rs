mod shader;
mod mesh;
mod meshes;
mod shaders;
mod texture;

use std::thread::sleep;
use std::time::Duration;
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use shader::*;
use shaders::*;
use mesh::*;
use meshes::*;
use texture::*;

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
    unsafe{ gl::Enable(gl::DEPTH_TEST) };

    /*// vertex shaders are guaranteed to have at least 16 vec4 inputs
    // you can check the actual amount using this
    // in my case it's still 16
    let mut res = 0;
    unsafe { gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut res) };
    println!("max vertex attributes {res}");*/

    let texture = Texture::new_from_jpeg("src/textures/container.jpg").ok_or(())?;
    let texture1 = Texture::new_from_png("src/textures/awesomeface.png").ok_or(())?;

    //let program_orange = program_orange().ok_or(())?;
    //let program = program_upside_down().ok_or(())?;
    //let program = program_offset().ok_or(())?;
    //let program_yellow = program_yellow().ok_or(())?;
    //let program = program_pos_colour().ok_or(())?;
    //let program = program_set_colour().ok_or(())?;
    //let program = program_in_colour().ok_or(())?;
    //let program = program_colour_and_texture().ok_or(())?;
    //let program = program_mvp().ok_or(())?;
    let program = program_mvp_texture().ok_or(())?;

    let (width, height) = window.get_framebuffer_size();
    window.set_framebuffer_size_callback(on_resize);
    on_resize(&mut window, width, height);

    //let mesh = rectangle();
    //let mesh = triangle();
    //let mesh = rectangle_screen();
    //let mesh = rectangle_texture();
    //let mesh = multi_attr_indices_interleaved();
    //let mesh = multi_attr_indices_batched();
    //let mesh = multi_attr_no_indices_interleaved();
    //let mesh = multi_attr_no_indices_batched();
    //let meshes = two_triangles_split();
    let mesh = cube();
    //unsafe{gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE)};

    let mut interpolation = 0.2;
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut time_start = glfw.get_time();
    let target = 1.0 / 60.0;
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    /*let scale = glam::Vec3::splat(1.25);
    let translation = glam::Vec3::new(0.25, 0.1, 0.0);
    let translation2 = glam::Vec3::new(-0.25, -0.1, 0.0);
    let translation_mat = glam::Mat4::from_translation(translation);
    let translation_mat2 = glam::Mat4::from_translation(translation2);*/
    let cube_positions = [
        glam::Vec3::new( 0.0,  0.0,  0.0),
        glam::Vec3::new( 2.0,  5.0, -15.0),
        glam::Vec3::new(-1.5, -2.2, -2.5),
        glam::Vec3::new(-3.8, -2.0, -12.3),
        glam::Vec3::new( 2.4, -0.4, -3.5),
        glam::Vec3::new(-1.7,  3.0, -7.5),
        glam::Vec3::new( 1.3, -2.0, -2.5),
        glam::Vec3::new( 1.5,  2.0, -2.5),
        glam::Vec3::new( 1.5,  0.2, -1.5),
        glam::Vec3::new(-1.3,  1.0, -1.5),
    ];
    let cube_translations = cube_positions.map(|x| glam::Mat4::from_translation(x));
    let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
    //let aspect_ratio = 1.777777;
    //let aspect_ratio = 0.5;
    while !window.should_close() {
        if window.get_key(Key::Escape) == Action::Press {
            window.set_should_close(true);
        }
        if window.get_key(Key::Up) == Action::Press {
            rot_x -= 0.1;
            if !up_pressed {
                interpolation += 0.1;
            }
            up_pressed = true;
        } else if window.get_key(Key::Up) == Action::Release {
            up_pressed = false;
        }
        if window.get_key(Key::Down) == Action::Press {
            rot_x += 0.1;
            if !down_pressed {
                interpolation -= 0.1;
            }
            down_pressed = true;
        } else if window.get_key(Key::Down) == Action::Release {
            down_pressed = false;
        }
        if window.get_key(Key::Left) == Action::Press {
            rot_y -= 0.1;
        }
        if window.get_key(Key::Right) == Action::Press {
            rot_y += 0.1;
        }
        let time_for_calc = time_start as f32;
        let sin = time_for_calc.sin();
        let sin_half = sin / 2.0;
        let model = glam::Mat4::from_rotation_x(rot_x)
            * glam::Mat4::from_rotation_y(rot_y);
        let view = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0));
        let projection = glam::Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);
        /*let transform = glam::Mat4::from_scale_rotation_translation(
            scale,
            glam::Quat::from_rotation_z(angle as f32),
            translation,
        );*/
        /*let transform = glam::Mat4::from_rotation_z(time_for_calc);
        let transform = translation_mat.mul_mat4(&transform);*/
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        program.use_program();
        //program.set_location_mat4f(0, &model);
        program.set_location_mat4f(1, &view);
        program.set_location_mat4f(2, &projection);
        program.set_texture(3, &texture);
        program.set_texture(4, &texture1);
        //program.set_location_mat4f(0, &transform);
        /*program.set_texture(0, &texture);
        program.set_texture(1, &texture1);*/
        //program.set_location_1f(2, interpolation);
        // to set the uniform value, you need to use the program first
        //program.set_location(0, 0.0, sin_half, 0.0, 1.0);
        //program.set_location(0, sin_half, 0.0, 0.0, 0.0);
        //mesh.render();
        for (index, &translation) in cube_translations.iter().enumerate() {
            let rot = if index % 3 == 0 {
                time_for_calc
            } else {
                index as f32 / 10.0
            };
            let model = translation * glam::Mat4::from_rotation_x(rot);
            program.set_location_mat4f(0, &model);
            mesh.render();
        }
        /*program_orange.use_program();
        meshes[0].render();
        program_yellow.use_program();
        meshes[1].render();*/
        /*let transform = glam::Mat4::from_scale(glam::Vec3::splat(sin_half));
        let transform = translation_mat2.mul_mat4(&transform);
        program.set_location_mat4f(0, &transform);
        mesh.render();*/

        window.swap_buffers();

        let end = glfw.get_time();
        let elapsed = end - time_start;
        time_start = end;
        /*let fps = 1.0 / elapsed;
        println!("{elapsed} {fps}");*/
        let left = target - elapsed;
        if left > 0.00001 {
            // it feels like this is not actually working correctly
            sleep(Duration::from_secs_f64(left));
        }

        glfw.poll_events();
    }
    Ok(())
}

fn on_resize(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe { gl::Viewport(0, 0, width.into(), height.into()) };
}


