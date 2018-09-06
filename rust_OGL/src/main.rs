

extern crate sdl2;
extern crate gl;

pub mod render_gl;

use std::ffi::{CString, CStr};



/*
    Erzeugt aus einem Sourcecode einen Shader und gibt die ShaderID als UInt Wert zurück
    @Args:
    source: Quellcode des Shaders
    kind: Art des Shaders
    @Return:
*/



fn main() {
    let sdl = sdl2::init().unwrap();            //Initialisiere sdl
    let video_subsystem = sdl.video().unwrap(); //Initialisiert das Video Subsystem

    let gl_attr = video_subsystem.gl_attr();    //Über dieses Objekt können Optionen gesetzt werden
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem                //Initialisiert den Windowbuilder und erstellt das Fenster
        .window("OGL-Test", 1024, 768)
        .opengl() //add opengl flag
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();    //Erstelle einen gl_Context
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);  //

    unsafe {
        gl::Viewport(0, 0, 1024, 768);              //Setze viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);         //Setzte die Clear Color Farbe      
    }

    use std::ffi::{CString};
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();
    let vertices: Vec<f32> = vec![
        // positions       // colors
         0.5, -0.5, 0.0,   1.0, 0.0, 0.0,   // bottom right
        -0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   // bottom left
         0.0,  0.5, 0.0,   0.0, 0.0, 1.0    // top
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }    
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
    }

    let mut event_pump = sdl.event_pump().unwrap(); //Über dieses Objekt können Events abgefangen werden (Eingaben vom Nutzer)

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {                                       //Abfrageschleife
                sdl2::event::Event::Quit {..} => break 'main,   //Beende das Programm bei Quit-Event
                _ => {},                                        //Tue nix bei anderen Events
            }
        }
        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        window.gl_swap_window();    //Tauscht den Windowbuffer
    }
}
