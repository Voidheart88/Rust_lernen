// Spice Class 
// Author: Dominik Stolte
// Starts a NGSpice Simulation and returns statistical data of output values

//represents a node as connection between circuit elements
extern crate libloading as lib;

#[derive(Debug)]
struct Node {
    name: String,
}

//represents a part - a Circuit element
#[derive(Debug)]
struct Part{
    nodes: Vec<Node>,
    name: String,
    value: String,
    model: String,
}

#[derive(Debug)]
struct Circuit{
    title: String,
    includes: Vec<String>,
    parts: Vec<Part>,
    options: Vec<String>,
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
    //Ctor
    pub fn new(c_title: String) -> Circuit {
        Circuit{
            title: c_title,
            includes: vec![],
            parts: vec![],
            options:vec![],
            models: vec![],
            directives: vec![],
        }
    }

    //Adds a Part
    pub fn add_part(&mut self,part:Part){
        self.parts.push(part)
    }

    pub fn print_netlist(&self){
        println!("{:?}",self.includes);
        println!("{:?}",self.parts);
        println!("{:?}",self.options);
        println!("{:?}",self.models);
        println!("{:?}",self.directives);
    }
    
    fn get_netstring(&self) -> String{
        let tit = &self.title;
        let opt = &self.options.join("\n");

        return String::from("bla");
    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn main(){
        println!("Start test");
        let mut c1 = Circuit::new(String::from("Testcircuit"));
        let n1 = Node::new(String::from("N001"));
        let n2 = Node::new(String::from("0"));
        c1.add_part(
            Part{
                nodes: vec![n1,n2],
                name: String::from("R1"),
                value: String::from("100"),
                model: String::from(""),
            }
        );
        println!("Netstring: {:?}",c1.get_netstring());
        let lib = call_dynamic();
        println!("{:?}",lib);
    }
}

/*
def getNetstring(self):
    #build preamble
    netlist: str = self.title
#add includes
    if len(self.includes):
        for line in self.includes:
            netlist+="\n"
            netlist+=(".include ")
            netlist+=line
    #add Parts
    for part in self.parts:
        netlist+="\n"
        netlist+=(part.name+" ")
        for node in part.nodes:
            netlist+=(node+" ")
        if part.value != None: netlist+=str(part.value)
        if part.model != None: netlist+=str(part.model)
    #add models
    if len(self.models) != 0:
        for model in self.models:
            netlist+="\n"
            netlist+=".model "
            netlist+=model+" "+model
    #add Spice Directives 
    if len(self.directives) != 0:
        for directive in self.directives:
            netlist+="\n"
            netlist+=directive   
    #add options
    if len(self.options) != 0:
        netlist+="\n.control"
        for option in self.options:
            netlist+="\n"
            netlist+=option
        netlist+="\n.endc"
    netlist+="\n.end"
    return netlist
*/