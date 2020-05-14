mod spice;
mod pso_mt;
mod vect;

use spice::{Circuit,Part,Node,Model};

fn main(){
	let mut ci = Circuit::new_circuit("modelfit");
        let mut p1 = Part::new_part(
            "V1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "DC 10", 
            Model::new_model("", String::from(""))
        );
        let mut p2 = Part::new_part(
            "D1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "", 
            Model::new_model("BYT03-400", String::from(""))
        );
        ci.add_part(&mut p1);
        ci.add_part(&mut p2);
        ci.add_tail(&String::from(".save @d1[id]"));
        ci.add_tail(&String::from(".dc V1 0 10 10m"));
        ci.add_tail(&String::from(".option filetype=ascii"));
        ci.change_part_model("D1",Model::new_model("BYT03-400", String::from("D BV=400 IBV=100 IS=1e-9 N=2 RS=0.05 VJ=0.7 CJO=0 M=0.33 TT=10e-9 EG=1.11")));
        //let deviations = vec![(0.0,0.0),(1.0,0.2040676898923585),(10.0,173.2268438722886)];
        //println!("{}",ci.get_rmsdev(deviations).unwrap());
}