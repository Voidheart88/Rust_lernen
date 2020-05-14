// Spice Class 
// Author: Dominik Stolte
// Implements an PSO-algorithm for functions f(x_a ... x_n) -> R^1
#![allow(dead_code)]
extern crate rand;
use rand::prelude::*;

#[derive(Debug)]
struct Particle{
	xi: (f64,f64),//position of particle
	vi: (f64,f64), //speedvector
	loc_min: (f64,f64,f64), //position and value of minimum
}

//Test functions:

//Test function Paraboloid:
// x_min = 0,0
fn paraboloid<T>((x_in,y_in):(T,T))-> T where	T: std::ops::Mul<Output = T>+ Copy, T: std::ops::Add<Output = T>{
	return  x_in*x_in + y_in*y_in
}

//Test function Paraboloid:
// f_min = -1
fn sine_x2y((x_in,y_in):(f64,f64))-> f64{
	return f64::sin(x_in*x_in+y_in)
}

//Test function Paraboloid:
// f_min = 1.1
fn rosenbrock((x_in,y_in):(f64,f64))-> f64{
	return (1.0-x_in)*(1.0-x_in)+(100.0*((y_in-x_in*x_in)*(y_in-x_in*x_in)));
}

//particle swarm optimization for functions of f: R^2 -> R
fn pso_2<F>(fun:F,r_tol:f64,a:f64,b1:f64,b2:f64) ->(f64,f64,f64) where F:Fn((f64,f64)) -> f64 {
	let mut rng = rand::thread_rng();
	let n_part = 5;
	let mut particles:Vec<Particle> = vec![];
	//Initialize Particles randomly in Searchspace
	for _ in 0..n_part{
		//x1,y1,vi
		particles.push(
			Particle{
			xi:(rng.gen_range(-10.0, 10.0),rng.gen_range(-10.0, 10.0)),
			vi:(0.0,0.0),
			loc_min:(f64::MAX,f64::MAX,f64::MAX),
		})
	}

	let mut x_min = (0.0,0.0,f64::MAX);
	let mut last_x_min = (0.0,0.0,f64::MAX);
	loop{
		//evaluate function and save position
		for particle in &mut particles{
			let fx = fun((particle.xi.0,particle.xi.1));
			//if fx < local min update local min
			if fx < particle.loc_min.2 {
				particle.loc_min = (particle.xi.0,particle.xi.1,fx);
			}
			//if fx < global min update global min
			if fx < x_min.2 {
				x_min = (particle.xi.0,particle.xi.1,fx);
				if (last_x_min.2 - x_min.2).abs() < r_tol {
					return x_min;
				}
				if last_x_min.2 > x_min.2 {
					last_x_min = x_min
				}
			}
		}
		
		for particle in &mut particles{
			//calculate new speed vector
			particle.vi = (
				a*particle.vi.0+b1*(particle.loc_min.0-particle.xi.0)+b2*(x_min.0-particle.xi.0),
				a*particle.vi.1+b1*(particle.loc_min.1-particle.xi.1)+b2*(x_min.1-particle.xi.1)
			);
			//calculate new position
			particle.xi = (
				particle.xi.0+particle.vi.0,
				particle.xi.1+particle.vi.1
			);
		}
	}
}

//Unit Tests
#[cfg(test)]
mod tests {
use crate::psa::{rosenbrock,paraboloid,sine_x2y,pso_2};

	#[test]
	fn test_fun(){
		println!("{:?}",pso_2(|x| paraboloid((x.0,x.1)),1e-3,1.0,0.1,0.2));
	}

	#[test]
	fn test_sinxy(){
		println!("{:?}",pso_2(|x| sine_x2y((x.0,x.1)),1e-3,1.0,0.1,0.2));
	}

	#[test]
	fn test_rosenbrock(){
		println!("{:?}",pso_2(|x| rosenbrock((x.0,x.1)),1e-3,1.0,0.01,0.02));
	}
}