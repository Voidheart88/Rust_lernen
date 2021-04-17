extern crate libloading;

use libloading::{Library, Symbol};

// int ngSpice_Init(SendChar*, SendStat*, ControlledExit*, SendData*,SendInitData*, BGThreadRunning*, void)
pub type Init = fn() -> i64;

//  int ngSpice_Command(char*)
pub type Command = fn() -> i64;

// bool ngSpice_running (void)
pub type Running = fn() -> bool;

// int ngSpice_Circ(char**)
pub type Circ = fn() -> i64;

// char* ngSpice_CurPlot(void)
pub type CurPlot = fn() -> *mut str;

#[derive(Debug)]
enum LibError {
    SymLoad,
    LibLoad,
}

struct Spice<'a> {
    lib:Library,
    init: Symbol<'a, Init>,
    command: Symbol<'a, Command>,
    running: Symbol<'a, Running>,
    circ: Symbol<'a, Circ>,
    cur_plot: Symbol<'a, CurPlot>,
}

impl<'a> Spice<'a> {
    unsafe fn new() -> Result<Spice<'a>, LibError> {
        let lib: Library = match Library::new("ngspice.dll") {
            Ok(o) => o,
            Err(_) => return Err(LibError::LibLoad),
        };
        let init: Symbol<Init> = match lib.get(b"ngSpice_Init") {
            Ok(o) => o,
            Err(_) => return Err(LibError::SymLoad),
        };
        let command: Symbol<Command> = match lib.get(b"ngSpice_Command") {
            Ok(o) => o,
            Err(_) => return Err(LibError::SymLoad),
        };
        let running: Symbol<Running> = match lib.get(b"ngSpice_running") {
            Ok(o) => o,
            Err(_) => return Err(LibError::SymLoad),
        };
        let circ: Symbol<Circ> = match lib.get(b"ngSpice_Circ") {
            Ok(o) => o,
            Err(_) => return Err(LibError::SymLoad),
        };
        let cur_plot: Symbol<CurPlot> = match lib.get(b"ngSpice_CurPlot") {
            Ok(o) => o,
            Err(_) => return Err(LibError::SymLoad),
        };

        Ok(Spice {
            lib: lib,
            init: init,
            command: command,
            running: running,
            circ: circ,
            cur_plot: cur_plot,
        })
    }
}
