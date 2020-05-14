#![allow(dead_code)]
extern crate rand;

use std::ops::{Add,Mul,Sub};
use generic_array::*;
use generic_array::typenum::*;
use rand::prelude::*;

#[derive(Debug,Clone)]
pub struct Vect<N: ArrayLength<f64>>{
	pub v: GenericArray<f64, N>
}
impl<N: ArrayLength<f64>> Vect<N> {
	pub fn new(val:f64) -> Vect<N>{
		let mut out:Vect<N> = Vect{v:GenericArray::default()};
		for i in 0..N::to_usize(){
			out.v[i] = val;
		}
		return out
	}
	pub fn new_rnd(min:f64,max:f64) -> Vect<N>{
		let mut rng = rand::thread_rng();
		let mut out:Vect<N> = Vect{v:GenericArray::default()};
		for i in 0..N::to_usize(){
			out.v[i] = rng.gen_range(min, max);
		}
		return out
	}
	pub fn expand(self,val:f64) -> Vect<N>{
		let mut out:Vect<N> = Vect{v:GenericArray::default()};
		for i in 0..N::to_usize(){
			out.v[i] = self.v[i]*val;
		}
		return out
	}
	pub fn abs(self) -> f64 {
		let mut out = 0.0;
		for i in 0..N::to_usize(){
			out += self.v[i]*self.v[i];
		}
		return out.sqrt();
	}
}

//impl<N: ArrayLength<f64> Copy for Vect<N> {	
//}

impl Vect<U3>{
	pub fn cross(self, other: Vect<U3>) -> Vect<U3>{
		return  Vect::<U3>{v:arr![f64;
			self.v[1]*other.v[2]-self.v[2]*other.v[1],
			self.v[2]*other.v[0]-self.v[0]*other.v[2],
			self.v[0]*other.v[1]-self.v[1]*other.v[0]
		]};
	}
}

impl<N: ArrayLength<f64>> Add for Vect<N>{
	type Output = Vect<N>;
	fn add(self, other: Vect<N>) -> Vect<N> {
		let mut out = Vect::<N>{v:GenericArray::default()};
		for i in 0..N::to_usize(){
			out.v[i] = self.v[i]+other.v[i];
		}
		return out
	}
}

impl<N: ArrayLength<f64>> Sub for Vect<N>{
	type Output = Vect<N>;
	fn sub(self, other: Vect<N>) -> Vect<N> {
		let mut out = Vect::<N>{v:GenericArray::default()};
		for i in 0..N::to_usize(){
			out.v[i] = self.v[i]-other.v[i];
		}
		return out
	}
}

//Implementation of Dot Product of two Vectors
impl<N: ArrayLength<f64>> Mul for Vect<N>{
	type Output = f64;
	fn mul(self, other: Vect<N>) -> f64 {
		let mut out = 0.0;
		for i in 0..N::to_usize(){
			out += self.v[i]*other.v[i];
		}
		return out
	}
}

#[cfg(test)]
mod tests {
use crate::vect::*;

	#[test]
	fn test_new(){
		let v1 = Vect::<U1>{v: GenericArray::default()};
		let v2 = Vect::<U2>{v: GenericArray::default()};
		println!("{:?}; {:?}",v1,v2)
	}

	#[test]
	fn test_add(){
		let v1 = Vect::<U2>{v: arr![f64;1.0,2.0]};
		let v2 = Vect::<U2>{v: arr![f64;1.0,2.0]};
		let v3 = v1+v2;
		assert_eq!(v3.v[0],2.0);
		assert_eq!(v3.v[1],4.0);
	}

	#[test]
	fn test_sub(){
		let v1 = Vect::<U2>{v: arr![f64;3.0,3.0]};
		let v2 = Vect::<U2>{v: arr![f64;1.0,2.0]};
		let v3 = v1-v2;
		assert_eq!(v3.v[0],2.0);
		assert_eq!(v3.v[1],1.0);
	}

	#[test]
	fn test_mul(){
		let v1 = Vect::<U3>{v: arr![f64;3.0,3.0,1.0]};
		let v2 = Vect::<U3>{v: arr![f64;1.0,2.0,1.0]};
		let v3 = v1*v2;
		assert_eq!(v3,10.0)
	}

	#[test]
	fn test_cross(){
		let v1 = Vect::<U3>{v: arr![f64;3.0,3.0,1.0]};
		let v2 = Vect::<U3>{v: arr![f64;1.0,2.0,1.0]};
		let v3 = v1.cross(v2);
		assert_eq!(v3.v[0],1.0);
		assert_eq!(v3.v[1],-2.0);
		assert_eq!(v3.v[2],3.0);
	}
}
