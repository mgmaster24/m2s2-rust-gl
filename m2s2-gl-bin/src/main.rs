use std::ffi::CString;

use m2s2_gl_lib::{program::Program, shaders::shader::Shader};

extern crate gl;
extern crate sdl2;

fn main() {
    let sdl2_context = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(s) => {
            println!("Failed to initialize SDL2. Error: {}", s);
            return;
        }
    };

    let video_subsys = match sdl2_context.video() {
        Ok(vs) => vs,
        Err(s) => {
            println!("Failed to initialize the video sub system. Error: {}", s);
            return;
        }
    };

    let gl_attr = video_subsys.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 2);

    let window = match video_subsys
        .window("M2S2 GL Lib Demo", 900, 700)
        .opengl()
        .resizable()
        .build()
    {
        Ok(win) => win,
        Err(win_err) => {
            println!("Failed to create the window. Error: {}", win_err);
            return;
        }
    };

    let mut event_pump = match sdl2_context.event_pump() {
        Ok(ep) => ep,
        Err(s) => {
            println!("Failed to create the event pump. Error: {}", s);
            return;
        }
    };

    let gl_context = match window.gl_create_context() {
        Ok(gl_c) => gl_c,
        Err(s) => {
            println!("Failed to create the OpenGL context. Error: {}", s);
            return;
        }
    };

    let _gl = gl::load_with(|s| video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        match Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()) {
            Ok(v) => v,
            Err(s) => {
                println!("Failed to load the vertex shader. Error: {}", s);
                return;
            }
        };

    let frag_shader =
        match Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()) {
            Ok(f) => f,
            Err(s) => {
                println!("Failed to load the fragment shader. Error: {}", s);
                return;
            }
        };
    let shader_prog = match Program::from_shaders(&[vert_shader, frag_shader]) {
        Ok(p) => p,
        Err(s) => {
            println!("Failed to link the shader program. Error: {}", s);
            return;
        }
    };

    shader_prog.set_used();

    let verts: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            verts.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            };
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}
