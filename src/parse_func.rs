#[path = "./structs/functions.rs"]
mod functions;
use functions::MathFunction;


pub fn main() {

    let x = Box::new( functions::SquareRoot(functions::Exp(functions::X, functions::Number(3.0))) );

    let mf = MathFunction::new(x);

    println!("{}", mf);

}