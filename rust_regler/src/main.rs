// Rust Regler
// 

//Control Loop
struct CLoop{

}

//Single Block
struct Block {
    transfer_fn: fn (Vec<f64>) -> f64,  //Transfer function
    internal: f64,                      //Internal Variable (needed for things like integrators)
}

fn transf_adder(a:f64,b:f64) -> f64 {
    a+b
}

fn transf_integrator(internal:f64,input:f64,delta:f64,gain:f64) -> f64 {
    internal+input*delta*gain
}

fn main() {
    println!("Hello, world!");
}
