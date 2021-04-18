extern crate gl;
extern crate sdl2;
extern crate tmmacro;
extern crate tmgl;

use std::ffi::{CStr, CString};
use tmmacro::file_name_and_content;

fn main() {

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let a = tmgl::shader_from_file(file_name_and_content!("triangle.frag"));
    let b = tmgl::shader_from_file(file_name_and_content!("triangle.vert"));

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        tmgl::clear(0.0, 0.0, 0.0, 0.0);

        window.gl_swap_window();
    }
}
