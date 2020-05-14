// Spice Class 
// Author: Dominik Stolte
// Starts a NGSpice Simulation and returns statistical data of output values
#![allow(dead_code)]
//#![allow(unused_variables)]

use std::process::{Command,Stdio};
use std::io::Write;
use std::f64;

#[derive(Debug,PartialEq,Clone)]
pub struct Part{
    name: &'static str,
    nodes: Vec<Node>,
    value: &'static str,
    model: Model,
}

impl Part{
    pub fn new_part(
        p_name:&'static str,
        p_nodes:Vec<Node>,
        p_value:&'static str,
        p_model:Model) -> Part{
        Part{
            name:p_name,
            nodes:p_nodes,
            value:p_value,
            model:p_model,
        }
    }

    pub fn change_model(&mut self,p_model:Model){
        self.model = p_model;
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Node{
    name:&'static str,
}

impl Node{
    pub fn new_node(n_name: &'static str) -> Node {
        Node{
            name: n_name,
        }
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Model{
    name:&'static str,
    value:String,
}

impl Model{
    pub fn new_model(m_name: &'static str,m_value: String) -> Model {
        Model{
            name: m_name,
            value: m_value,
        }
    }
}

#[derive(Debug)]
pub struct Circuit<'a>{
    head: Vec<String>,
    parts: Vec<&'a mut Part>,
    models: Vec<String>,
    directives: Vec<String>,
    tail: Vec<String>,
    values: Vec<(f64,f64)>,
}

impl<'a> Circuit<'a>{
    pub fn new_circuit(n_title: &'static str) -> Circuit{
        Circuit{
            head: vec![n_title.to_string()],
            parts: vec![],
            models: vec![],
            directives: vec![],
            tail: vec![],
            values: vec![],
        }
    }

    pub fn add_part (&mut self,part:&'a mut Part){
        if !(self.parts.iter().any(|x| *x  == part)){
            &self.parts.push(part);
        }
    }

    pub fn add_tail(&mut self,input:&String){
        if !(self.tail.iter().any(|x| *x  == input.clone())){
            &self.tail.push(input.clone());
        }
    }

    pub fn change_part_model(&mut self,p_name:&str,p_model:Model){
        for part in &mut self.parts{
            if part.name == p_name {
                part.change_model(p_model.clone());
            }
        }
    }

    pub fn get_netlist(&self) -> String{
        let mut netlist = String::new();
        netlist += ".title ";
        //Add Header
        netlist += &self.head.iter().fold(String::new(), |acc, x| acc + &x+"\n");
        //Add Parts
        for part in &self.parts{
            netlist += part.name;
            netlist += " ";
            for node in &part.nodes{
                netlist += node.name;
                netlist += " ";
            }
            if part.value != ""{
                netlist += part.value;
                netlist += " ";
            }
            if part.model.name != ""{
                netlist += &part.model.name;
                netlist += " ";
            }
            netlist += "\n";
        }
        //Add Tail
        for part in &self.parts {
            if part.model.name != ""{
                netlist += ".model";
                netlist += " ";
                netlist += &part.model.name;
                netlist += " ";
                netlist += &part.model.value;
                netlist += "\n";
            }
        }
        for tail in &self.tail{            
            netlist += tail;
            netlist += "\n";
        }
        //Add end
        netlist += ".end";
        netlist += "\n";
        return netlist;
    }

    fn get_values(&mut self) -> Result<bool, &'static str>{
        let mut child = Command::new("ngspice_con.exe")
            .args(&["-s"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(self.get_netlist().as_bytes()).expect("Failed to write to stdin");

        let output = String::from_utf8(child.wait_with_output().expect("Failed to read stdout").stdout);
        let mut output_ok = String::new();
        match output{
            Ok(v) => {
                output_ok = v;
            },
            Err(v) => println!("Error: {}",v),
        }
        // get Lines
        let mut lines: Vec<&str> = output_ok.lines().collect();

        //Concat elements till "Values:"
        let mut index = 0;
        for i in 0..lines.len() {
            if lines[i] == "Values:"{
                index = i;
                break
            }
        }
        if index == 0 {
            return Err("Error: No Values");
        }
        lines.drain(0..index+1);
        let mut vals: Vec<f64>= Vec::new();
        for line in lines {
            let tokens: Vec<&str> = line.split("\t").collect();
            let val = tokens.last().unwrap().parse::<f64>().unwrap();
            vals.push(val);
        }
        let times:Vec<f64> = vals.iter().step_by(2).map(|&x| x).collect();
        let voltages:Vec<f64> = vals.iter().skip(1).step_by(2).map(|&x| x).collect();
        for i in 0..voltages.len(){
            &self.values.push((times[i],voltages[i]));
        }
        return Ok(true)
    }

    pub fn get_rms(&mut self) -> Result<f64, &'static str>{
        let _res = self.get_values().expect("Error: no Values");
        let mut square_sum = 0.0;
        for value in &self.values {
            square_sum += value.1*value.1;
        }
        square_sum = square_sum.sqrt();
        Ok(square_sum)
    }

    //returns the sum of deviations to points given
    pub fn get_rmsdev(&mut self,pattern:Vec<(f64,f64)>) -> Result<f64, &'static str>{
        if self.values.len() == 0{
            let _res = self.get_values()?; // Question mark means: if error - function returns error
        }
        let mut deviations:f64 = 0.0;
        for entry in pattern{
            let mut devs:Vec<(f64,usize)> = vec![];
            //Calculate time deviation
            let mut i:usize = 0;
            for val in &self.values{
                let x_dev = entry.0-val.0;
                &devs.push((x_dev.abs(),i));
                i +=1;
            }
            devs.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());            
            //push entry - deviation
            let fac = entry.1-self.values[devs[0].1].1;
            deviations += fac*fac;
        }
        Ok(deviations)
    }
}

//Unit Tests
#[cfg(test)]
mod tests {
    use crate::*;
    //Test: 
    // the Netlist - Generate some Parts and build Head and Tail
    #[test]
    fn test1(){
        let mut c1 = Circuit::new_circuit("Testcircuit");
        let mut p1 = Part::new_part("V1", vec!(Node::new_node("N001"),Node::new_node("0")), "10", Model::new_model("", String::from("")));
        let mut p2 = Part::new_part("R1", vec!(Node::new_node("N001"),Node::new_node("0")), "10", Model::new_model("", String::from("")));
        c1.add_part(&mut p1);
        c1.add_part(&mut p2);
        c1.add_tail(&String::from(".option filetype=ascii"));
        c1.add_tail(&String::from(".save V(N001)"));
        c1.add_tail(&String::from(".tran 10m 1"));
        //println!("{}",c1.get_netlist());
        let _res = c1.get_values().expect("Error - no Values");
        //Test some Values
        assert_eq!(c1.values[0],(0.0, 10.0));
        assert_eq!(c1.values[1],(0.0001, 10.0));
        assert_eq!(c1.values[50],(0.4328000000000002, 10.0));
        assert_eq!(c1.values[100],(0.9328000000000006, 10.0));
    }

    //Test index bounds
    #[test]
    #[should_panic]
    fn test2(){
        let mut c1 = Circuit::new_circuit("Testcircuit");
        let mut p1 = Part::new_part("V1", vec!(Node::new_node("N001"),Node::new_node("0")), "10", Model::new_model("", String::from("")));
        let mut p2 = Part::new_part("R1", vec!(Node::new_node("N001"),Node::new_node("0")), "10", Model::new_model("", String::from("")));
        c1.add_part(&mut p1);
        c1.add_part(&mut p2);
        c1.add_tail(&String::from(".option filetype=ascii"));
        c1.add_tail(&String::from(".save V(N001)"));
        c1.add_tail(&String::from(".tran 10m 1"));
        c1.add_tail(&String::from(".end"));
        let _res = c1.get_values();
        //Test last value
        let _last = c1.values[108];
    }

    //Test index bounds
    #[test]
    fn test3(){
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
        ci.parts[1].model = Model::new_model("BYT03-400", String::from("D BV=400 IBV=100 IS=1e-9 N=2 RS=0.05 VJ=0.7 CJO=0 M=0.33 TT=10e-9 EG=1.11"));
        //println!("{}",ci.get_netlist());
        let _res = ci.get_values().unwrap();
    }

    //Test square Root
    #[test]
    fn test_rms(){
        let mut ci = Circuit::new_circuit("modelfit");
        let mut p1 = Part::new_part(
            "V1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "DC 10", 
            Model::new_model("", String::from(""))
        );
        let mut p2 = Part::new_part(
            "R1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "10", 
            Model::new_model("", String::from(""))
        );
        ci.add_part(&mut p1);
        ci.add_part(&mut p2);
        ci.add_tail(&String::from(".save V(N001)"));
        ci.add_tail(&String::from(".tran 1 10"));
        ci.add_tail(&String::from(".option filetype=ascii"));
        let rms = ci.get_rms().unwrap();
        assert_eq!(rms,76.81145747868608);
        assert_ne!(rms,0.0);
    }

    //Test square Root
    #[test]
    fn test_rmsdev1(){
        let mut ci = Circuit::new_circuit("modelfit");
        let mut p1 = Part::new_part(
            "V1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "DC 10", 
            Model::new_model("", String::from(""))
        );
        let mut p2 = Part::new_part(
            "R1", 
            vec!(Node::new_node("N001"),Node::new_node("0")), 
            "10", 
            Model::new_model("", String::from(""))
        );
        ci.add_part(&mut p1);
        ci.add_part(&mut p2);
        ci.add_tail(&String::from(".save V(N001)"));
        ci.add_tail(&String::from(".tran 1 10"));
        ci.add_tail(&String::from(".option filetype=ascii"));
        let deviations = vec![(0.0,0.0),(1.0,10.0),(2.0,100.0)];
        let dev = ci.get_rmsdev(deviations).unwrap();
        assert_eq!(dev,8200.0);
    }

    //Test square Root
    #[test]
    fn test_rmsdev2(){
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
        ci.add_tail(&String::from(".dc V1 0 10 100m"));
        ci.add_tail(&String::from(".option filetype=ascii"));
        ci.parts[1].model = Model::new_model("BYT03-400", String::from("D BV=400 IBV=100 IS=1e-9 N=2 RS=0.05 VJ=0.7 CJO=0 M=0.33 TT=10e-9 EG=1.11"));
        let deviations = vec![(0.0,0.0),(1.000000000000001,0.2040676898923585),(9.999999999999831,173.2268438722886)];
        let dev = ci.get_rmsdev(deviations).unwrap();
        assert_eq!(dev,0.0000000000024357310294397954)
    }
}