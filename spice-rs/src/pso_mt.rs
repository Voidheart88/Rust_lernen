#![allow(dead_code)]

extern crate rand;
extern crate crossbeam;
extern crate bus;

use std::sync::atomic::AtomicBool;
#[allow(unused_imports)]
use std::thread;
use std::thread::sleep;
use std::time::{Duration};
use bus::Bus;
use crossbeam::crossbeam_channel::unbounded;

use crate::vect::{Vect};

use rand::prelude::*;

use generic_array::*;
use generic_array::typenum::*;


pub struct Swarm<N: ArrayLength<f64>>{
	min: (Vect::<N>,f64),	//Position and value of global minimum
	tol: f64,							//tolerance for abort condition
	stop: bool, 					//Stop flag
	coeff: (f64,f64,f64),	//Coefficients
	u_barrier: Vect::<N>,	//upper barrier
	l_barrier: Vect::<N>,	//lower barrier
	gmin_c: (crossbeam::Sender<(Vect::<N>,f64)>,crossbeam::Receiver<(Vect::<N>,f64)>),
	state_s: bus::Bus<(f64,f64,f64,bool)>,
}

#[derive(Debug)]
pub enum PsoErr{
	TolBelowZero,
}

struct Particle<N: ArrayLength<f64>>{
	xi: Vect::<N>,															//Position of particle
	vi: Vect::<N>,															//Speedvector
	loc_min: (Vect::<N>,f64),										//Position and value of local minimum
	gmin: (Vect::<N>,f64),											//Position and value of global minimum
	state: (f64,f64,f64,bool),									//state of the particle
	gmin_c: (crossbeam::Sender<(Vect::<N>,f64)>,crossbeam::Receiver<(Vect::<N>,f64)>),//Channel for possible global minimum	//Sender for possible global minimum
	state_r: bus::BusReader<(f64,f64,f64,bool)>,//Receiver for state changes
}

impl<N: 'static +  ArrayLength<f64>> Swarm<N> {
	pub fn new(a:f64,b1:f64,b2:f64,tol:f64,opt_u_barrier:Option<Vect<N>>,opt_l_barrier:Option<Vect<N>>) -> Result<Swarm<N>,PsoErr> {
		let mut rng = rand::thread_rng();
		let mut u_barrier = Vect::new(f64::MAX);
		let mut l_barrier = Vect::new(f64::MAX);
		let (gmin_s,gmin_r) = unbounded();
		let state_s = Bus::<(f64,f64,f64,bool)>::new(10);
		if tol < 0.0 {
			return Err(PsoErr::TolBelowZero);
		}
		if let Some(barrier) = opt_u_barrier{
			u_barrier = barrier;
		}
		if let Some(barrier) = opt_l_barrier{
			l_barrier = barrier;
		}
		return Ok(Swarm{
			min: (Vect::new(f64::MAX),rng.gen()),	//Position and value of global minimum at random spot, will be overwritten after first function evaluation
			stop: false, 						//Stop condition
			tol: tol,								//Tolerance for abort condition
			coeff: (a,b1,b2),				//Coefficients
			u_barrier: u_barrier,		//Upper barriers
			l_barrier: l_barrier,		//Lower barriers
			gmin_c: (gmin_s,gmin_r),//Channel for possible global minimum
			state_s: state_s,				//Sender for coefficients
		})
	}
	pub fn solve<F: 'static>(&mut self,fun:F,opt_n:Option<usize>,opt_coeff:Option<(f64,f64,f64)>) -> Result<(Vect<N>,f64),PsoErr>  where F: std::marker::Send +Fn(&Vect<N>) -> f64{
		//check parameter
		let mut swarm_size = 0;
		let mut coeff = (1.0,0.5,0.5);
		if let Some(o) = opt_n{
			swarm_size = o;
		}
		else {
			swarm_size = N::to_usize()*5;
		}
		if let Some(o) = opt_coeff{
			coeff = o;
		}
		//Build swarm
		for i in 0..swarm_size{
			let particle = Particle::new(self.gmin_c.clone(), self.state_s.add_rx(), None, None);
			thread::spawn(move ||{
				particle.work(fun, i as i64)
			});
		}
		return Ok((Vect::new(0.0),0.0))
	}
}

impl<N: ArrayLength<f64>> Particle<N> {
	pub fn new(
			gmin_c: (crossbeam::Sender<(Vect::<N>,f64)>,crossbeam::Receiver<(Vect::<N>,f64)>),
			state_r: bus::BusReader<(f64,f64,f64,bool)>,
			opt_u_barrier:Option<Vect<N>>,opt_l_barrier:Option<Vect<N>>
		) -> Particle<N>{
		let mut rng = rand::thread_rng();
		//calculate start position
		let mut xi = Vect::<N>::new(0.0);
		let mut u_barrier = Vect::<N>::new(f64::MAX);
		let mut l_barrier = Vect::<N>::new(-f64::MAX);
		if let Some(barrier) = opt_u_barrier{
			u_barrier = barrier
		}
		if let Some(barrier) = opt_l_barrier{
			l_barrier = barrier
		}
		for i in 0..N::to_usize(){
			xi.v[i] = rng.gen_range(l_barrier.v[i], u_barrier.v[i])
		}
		Particle{
			xi: xi,																				//Position of, randomly in searchspace
			vi: Vect::<N>::new(0.0),											//Speedvector
			loc_min: (Vect::<N>::new(rng.gen()),f64::MAX),//Position and value of local minimum
			gmin: (Vect::<N>::new(rng.gen()),f64::MAX),		//Position and value of global minimum
			state: (1.0,0.5,0.5,false),										//state of the particle
			gmin_c: gmin_c,																//Sender for possible global minimum
			state_r: state_r,															//Receiver for state changes
		}
	}

	pub fn work<F>(&mut self,fun:F,id:i64) where F:Fn(&Vect<N>) -> f64 {
		loop{
			//Update State
			match self.state_r.recv() {
				Ok(v) => self.state = v,
				Err(_) => break,
			}
			//if stop break
			if self.state.3 == true {break}
			//check if new gmin
			match self.gmin_c.1.recv(){
				Ok(V) => self.gmin = V,
				Err(_) => break,
			}
			let fx = fun(&self.xi);
			//if fx < local min update local min
			if fx < self.loc_min.1 {
				self.loc_min = (self.xi.clone(),fx);
			}
			//if fx < global min send new Value to global min
			if fx < self.gmin.1 {
				match self.gmin_c.0.send((self.xi.clone(),fx)){
					Ok(_)=> (),
					Err(_)=> break,
				}
			}

			//calculate new speed vector
			self.vi = self.vi.clone().expand(self.state.0)+(self.loc_min.clone().0-self.xi.clone()).expand(self.state.1)+(self.gmin.0.clone()-self.xi.clone()).expand(self.state.2);
			
			//calculate new position
			self.xi = self.xi.clone()+self.vi.clone();
		}
	}
}

fn paraboloid(inp:&Vect<U2>)-> f64{
	return  inp.v[0]*inp.v[0] + inp.v[1]*inp.v[1]
}

//Test function Rosenbrock:
// f_min = 0
// (x,y) = 1,1
fn rosenbrock(inp:&Vect<U2>)-> f64{
	return (1.0-inp.v[0])*(1.0-inp.v[0])+(100.0*((inp.v[1]-inp.v[0]*inp.v[0])*(inp.v[1]-inp.v[0]*inp.v[0])));
}

//Unit Tests
#[cfg(test)]
mod tests {
use crate::pso_mt::*;

	#[test]
	fn test_ctor(){
		let mut _swarm;
		match Swarm::<U2>::new(1.0,0.5,0.5,0.1,None,None) {
			Ok(o) =>  _swarm = o,
			Err(e) => println!("Error by building the swarm: {:?}",e),
		}
	}
	#[test]
	fn test_s_build(){
		let mut swarm = Swarm::<U2>::new(1.0,0.5,0.5,0.1,None,None).unwrap();
		let solution = swarm.solve(rosenbrock,None,None);
		println!("{:?}",solution)
	}
}