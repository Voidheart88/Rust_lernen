use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;

static DEFAULT_RC: &str = "PATH_BJT = ~/.spilib/bjt.lib
PATH_DIODE = ~/.spilib/diode.lib
PATH_JFET = ~/.spilib/jfet.lib
PATH_MOSFET = ~/.spilib/mosfet.lib";

static OPTION_PATH: &str = ".spilibrc";

fn main() {
    let o_path = OPTION_PATH;
    // Open Options File in standart path
    // if not present -> create one
    let f = match File::open(o_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let mut fc = match File::create(o_path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                };
                fc.write_all(DEFAULT_RC.as_bytes()).unwrap();
                drop(fc);
                let fc = File::open(o_path).unwrap();
                fc
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    // Read Options
    let options = fs::read_to_string(o_path).unwrap();
    print!("{}", options);
}
