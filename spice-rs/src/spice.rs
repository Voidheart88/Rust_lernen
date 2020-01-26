// Spice Class 
// Author: Dominik Stolte
// Starts a NGSpice Simulation and returns statistical data of output values

//represents a node as connection between circuit elements

#[derive(Debug,PartialEq,Clone)]
struct Part{
    name: &'static str,
    nodes: Vec<&'static str>,
    value: &'static str,
    model: &'static str,
}

impl Part{
    fn new(p_name:&'static str,p_nodes:Vec<&'static str>,p_value:&'static str,p_model:&'static str) -> Part{
        Part{
            name:p_name,
            nodes:p_nodes,
            value:p_value,
            model:p_model,
        }
    }
}

#[derive(Debug)]
enum Mode{
    TRAN,
    AC,
}

#[derive(Debug)]
struct Circuit{
    head: Vec<String>,
    parts: Vec<Part>,
    models: Vec<String>,
    directives: Vec<String>,
}

struct NGSpice{
    init: unsafe extern fn() -> u32,
}

impl NGSpice{
    pub fn new () -> NGSpice {
        NGSpice{
            init: NGSpice::call_init(),

        }
    }
    fn call_init() -> lib::Result<u32> {
        let lib = lib::Library::new("./lib/ngspice.dll")?;
        unsafe {
            let init: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"ngSpice_Init")?;
            Ok(init())
        }
    }
}


impl Node{
    pub fn new(n_name: String) -> Node {
        Node{
            name: n_name,
        }
    }
}

impl Part{
    //Constructor
    pub fn new(p_nodes: Vec<Node>, p_name: String, p_value: String, p_model: String) -> Part {
        Part{
            nodes: p_nodes,
            name: p_name,
            value: p_value,
            model: p_model,
        }
    }
}

impl Circuit{
    pub fn new(n_title: String) -> Circuit{
        Circuit{
            head: vec![n_title],
            parts: vec![],
            models: vec![],
            tail: vec![],
            mode: Mode::TRAN,
            tran: ("1m".to_string(),"1".to_string()),
        }
    }

    pub fn add_part(&mut self,part:&Part){
        if !(self.parts.iter().any(|x| *x  == part.clone())){
            &self.parts.push(part.clone());
        }
    }

    pub fn set_tran(&mut self,step:String,dur:String){
        self.mode = Mode::TRAN;
        self.tran=(step,dur);
    }

    pub fn add_tail(&mut self,input:&String){
        if !(self.tail.iter().any(|x| *x  == input.clone())){
            &self.tail.push(input.clone());
        }
    }

    pub fn get_netlist(&self) -> String{
        let mut netlist = String::new();
        netlist += ".title ";
        netlist += &self.head.iter().fold(String::new(), |acc, x| acc + &x+"\n");
        netlist += &self.parts
            .iter()
            .map(|x|x.name.to_string()+" "+x.nodes+" "+x.value+" "+x.model+"\n")
            .collect::<Vec<String>>()
            .join("");
        match &self.mode{
            Mode::TRAN => {
                netlist += ".tran ";
                netlist += &self.tran.0;
                netlist += " ";
                netlist += &self.tran.1;
                netlist += "\n";
            },
            _ => {
                netlist += ".tran 1m 1 \n";
            },
        }
        netlist += ".control\n";
        netlist += &self.tail.iter().fold(String::new(), |acc, x| acc + &x+"\n");
        netlist += ".endc\n";
        netlist += ".end\n";
        netlist
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn main(){
        let p1 = Part::new("V1",vec!["N001","0"],"10","");
    }
}