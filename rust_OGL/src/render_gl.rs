use gl;
use std;
use std::ffi::{CString, CStr};

//Schaderprogramm welches von der Grafikkarte ausgeführt wird
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram()}; //Erstelle ein Shader Programm mit einer ID

        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id());} //Füge die kompilierten Shader an das Programm an
        }

        unsafe { gl::LinkProgram(program_id); } //Programm Linken

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); //Kompiliere das Programm
        }

        if success == 0 {   //Wenn der Kompiliervorgang fehlschlägt, erstelle die Fehlermeldung und gebe sie als Error zurück
            let mut len: gl::types::GLint = 0;  //Initialisiere die Länge
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);    //hole die länge Fehlermenldung
            }

            let error = create_whitespace_cstring_with_len(len as usize);   //Speichere die Fehlermeldung in Error

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned()); //Gib den Error zurück
        }

        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); } //gib die Ressourcen wieder frei
        }

        Ok(Program { id: program_id })

    }
    // gibt die ID des Shaders zurück
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
    //aktiviert den Shader
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

}

//"Destruktor"
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

//Shaderobjekt
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    //Konstruktor allgemein
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }
    //Konstruktor Vertex Shader
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    //Konstruktor Fragment Shader
    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_geom_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::GEOMETRY_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

//"Destruktor"
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

//Hilfsfunktion um Shader aus einer Quelle zu laden
fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

//Hilfsfunktion um einen String mit einer Länge zu erstellen
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}