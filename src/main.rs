mod polynomial;
use polynomial::{Polynomial, PolynomialStringError};
use std::io::stdin;

fn main() -> Result<(), PolynomialStringError> {
    println!("Polynomial multiplier: for students, teachers, and everyone else. If you're avoiding work, you're in the right place.\n");
    println!("Here's the how it should look in case you forgot:\n(3x^3 + x^2 + 26x - 5)(x^2 + 4x)(x^16 - 12x^9 + -2)... etc.");
    println!("So go ahead:\n");

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let poly_string = buf.trim_end_matches(|c| c == '\n' || c == '\r');

    let poly = poly_string.parse::<Polynomial>()?;
    println!("{:?}", poly);
    println!{"{}", poly};

    Ok(())
}
