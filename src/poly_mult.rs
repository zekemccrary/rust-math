#[path = "./utils/polynomial.rs"]
mod polynomial;

use polynomial::{Polynomial, PolynomialStringError};
use std::io::stdin;

pub fn main() -> Result<(), PolynomialStringError> {
    println!("Polynomial multiplier:");
    println!("Here's the how it should look in case you forgot:\n(3x^3 + x^2 + 26x - 5)(x^2 + 4x)(x^16 - 12x^9 + -2)... etc.");
    println!("So go ahead:\n");

    // read input from user
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let poly_string = buf.trim_end_matches(|c| c == '\n' || c == '\r');

    let poly = parse_poly_mult_string(&poly_string)?.simplify().organize();

    println!("{:?}", poly);
    println!("{}", poly);
    
    Ok(())
}

fn parse_poly_mult_string(ps: &str) -> Result<Polynomial, PolynomialStringError> {
    const OPEN_PARENTHESIS: bool = true;
    const CLOSE_PARENTHESIS: bool = false;

    let mut build: Polynomial = Polynomial::new( vec![ [1.0, 0.0] ] ); // default to multiply by

    // which parens is expected
    let mut parens_open = false;
    // index of the open parenthesis
    let mut start: usize = 0;

    for (i, c) in ps.chars().enumerate() {
        match c {
            // should be a close parenthesis
            '(' if parens_open  => return Err(PolynomialStringError::ParenthesesError(i, OPEN_PARENTHESIS)),
            ')' if !parens_open => return Err(PolynomialStringError::ParenthesesError(i, CLOSE_PARENTHESIS)),
            '('                 => { parens_open = true; start = i; },

            ')' => { 
                parens_open = false;
                build = build.mult( &ps[start + 1 .. i].parse::<Polynomial>()? );
            },

            _ => continue,
        }
    }

    Ok(build)
}