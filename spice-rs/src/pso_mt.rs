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
	gmin_r: crossbeam::Receiver<(Vect::<N>,f64)>,	//Receiver for possible global minimum
	coeff_s: bus::Bus<(f64,f64,f64,bool)>,
}

struct Particle<N: ArrayLength<f64>>{
	xi: Vect::<N>,															//Position of particle
	vi: Vect::<N>,															//Speedvector
	loc_min: (Vect::<N>,f64),										//Position and value of local minimum
	gmin: (Vect::<N>,f64),											//Position and value of global minimum
	stop: bool,																	// stop state, if true -> stop calc
	coeff: (f64,f64,f64),												//
	gmin_s: crossbeam::Sender<(Vect::<N>,f64)>,	//Sender for possible global minimum
	coeff_r: bus::BusReader<(f64,f64,f64,bool)>,//Receiver for state changes
}

impl<N: ArrayLength<f64>> Swarm<N> {
	pub fn new(a:f64,b1:f64,b2:f64,tol:f64,gmin_r:crossbeam::Receiver<(Vect::<N>,f64)>,coeff_s:bus::Bus<(f64,f64,f64,bool)>,opt_u_barrier:Option<Vect<N>>,opt_l_barrier:Option<Vect<N>>) -> Option<Swarm<N>> {
		let mut rng = rand::thread_rng();
		let mut u_barrier = Vect::new(f64::MAX);
		let mut l_barrier = Vect::new(f64::MAX);
		if tol < 0.0 {
			return None;
		}
		if let Some(barrier) = opt_u_barrier{
			u_barrier = barrier;
		}
		if let Some(barrier) = opt_l_barrier{
			l_barrier = barrier;
		}
		return Some(Swarm{
			min: (Vect::new(f64::MAX),rng.gen()),	//Position and value of global minimum at random spot, will be overwritten after first function evaluation
			stop: false, 					//Stop condition
			tol: tol,							//Tolerance for abort condition
			coeff: (a,b1,b2),			//Coefficients
			u_barrier: u_barrier,	//Upper barrier
			l_barrier: l_barrier,	//Lower barrier
			gmin_r: gmin_r,				//Receiver for possible global minimum
			coeff_s: coeff_s,			//Sender for coefficients
		})
	}
	pub fn build(){
		unimplemented!();
	}
}

impl<N: std::fmt::Debug + ArrayLength<f64>> Particle<N> {
	pub fn new() -> Particle<N>{
		Particle{
			xi: xi_in,
			vi: Vect::<N>::new(0.0),
			loc_min: (Vect::<N>::new(0.0),f64::MAX),
			gmin: gmin,
			stop: stop,
			sender: sender,
			coeff: coeff,
		}
	}

	pub fn work<F>(&mut self,fun:F,id:i64) where F:Fn(&Vect<N>) -> f64 {
		println!("start Vector: {:?}:{:?}", id,self.xi);
		loop{
			let fx = fun(&self.xi);
			//if fx < local min update local min
			if fx < self.loc_min.1 {
				self.loc_min = (self.xi.clone(),fx);
				
			}
			//if fx < global min send new Value to global min
			let mut val_gmin = f64::MAX;
			let mut vec_gmin = Vect::new(0.0);
			if let Ok(gmin) = self.gmin.read() {
				val_gmin = gmin.1;
				vec_gmin = gmin.0.clone();
			}
			if fx < val_gmin {
				if let Ok(mut write_guard) = self.gmin.write() {
					// the returned write_guard implements `Deref` giving us easy access to the target value
					*write_guard = (self.vi.clone(),fx);
					}
			}
			//sleep(Duration::from_millis(500));
			//calculate new speed vector
			let coeff = self.coeff.load();
			self.vi = self.vi.clone().expand(coeff.0)+(self.loc_min.clone().0-self.xi.clone()).expand(coeff.1)+(vec_gmin.clone()-self.xi.clone()).expand(coeff.2);
			
			//calculate new position
			self.xi = self.xi.clone()+self.vi.clone();
			//if stop, stop the run
			if self.stop.load(Ordering::Relaxed) == true {break}
			//send the distance of the particle to the main thread.Arc
			let distance = (vec_gmin.clone()-self.xi.clone()).abs();
			self.sender.send((id,distance));
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
		let swarm = Swarm::new();
		let (sender, receiver) = mpsc::channel();

		let _particle1 = Particle::new(Vect::<U3>::new(1.0),swarm.min.clone(),swarm.stop.clone(),sender.clone(),&swarm.coeff);
		let _particle2 = Particle::new(Vect::<U3>::new(1.0),swarm.min.clone(),swarm.stop.clone(),sender.clone(),&swarm.coeff);
	}

	#[test]
	fn test_build_swarm(){
		//Build global minimum storage
		let swarm = Swarm::<U2>::new();
		let (sender, receiver) = mpsc::channel();
		const N_PARTICLES: i64 = 10;
		const RTOL:f64 = 0.0001;
		//Build swarm
		println!("Build Swarm");
		let mut threads = vec![];
		for i in 0..N_PARTICLES{
			let mut rng = rand::thread_rng();
			let mut particle = Particle::new(Vect::<U2>::new(rng.gen()),swarm.min.clone(),swarm.stop.clone(),sender.clone(),&swarm.coeff);
			threads.push(
				thread::spawn(move || {
					particle.work(rosenbrock,i);
				})
			)
		}

		//
		println!("Start Loop");
		let mut distances:Vec<f64> = vec![];
		for i in 0..N_PARTICLES{
			let n = N_PARTICLES as f64;
			distances.push(f64::MAX/n);
		}
		loop{
			let distance = receiver.recv().unwrap();
			distances[distance.0 as usize] = distance.1;
			let mut sum:f64 = distances.iter().sum();
			sum /= N_PARTICLES as f64;
			swarm.coeff.store((
				1.0,
				0.5,
				0.5,
			));
			println!("sum of distances {}",sum);
			if sum < RTOL {
				swarm.stop.store(true, Ordering::Relaxed);
				break;
			}
		}
		for thread in threads{
			thread.join();
		}
		println!("finish: {:?}",swarm.min);
	}
}