

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
    
    
    let mut event_pump = sdl.event_pump().unwrap(); //Über dieses Objekt können Events abgefangen werden (Eingaben vom Nutzer)

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {                                       //Abfrageschleife
                sdl2::event::Event::Quit {..} => break 'main,   //Beende das Programm bei Quit-Event
                _ => {},                                        //Tue nix bei anderen Events
            }
        }
        unsafe {gl::Clear(gl::COLOR_BUFFER_BIT);}   //Setze den 

        window.gl_swap_window();    //Tauscht den Windowbuffer
    }
}
