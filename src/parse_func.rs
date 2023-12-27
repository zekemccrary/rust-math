#[path = "./structs/functions.rs"]
mod functions;

pub fn main() {
    let func = functions::Number(3.14);

    println!("Number(3.14): {}", func);
}

/*

let func = OpFunc::Add(
        OpFunc::Multiply(
            Number(3.14), X
        ),
        Number(4.2)
    );

*/