mod polynomial;
use polynomial::{Polynomial, PolynomialStringError};
// use std::io::stdin;

fn main() -> Result<(), PolynomialStringError> {
    println!("Polynomial multiplier:");
    println!("Here's the how it should look in case you forgot:\n(3x^3 + x^2 + 26x - 5)(x^2 + 4x)(x^16 - 12x^9 + -2)... etc.");
    println!("So go ahead:\n");

    /*
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let poly_string = buf.trim_end_matches(|c| c == '\n' || c == '\r');

    let poly = poly_string.parse::<Polynomial>()?;
    println!("{:?}", poly);
    println!{"{}", poly};
    */

    let poly1 = Polynomial::new(  vec![ [2.4, 3.3], [1.5, 0.7], [4.2, -1.1] ]  );
    let poly2 = Polynomial::new(  vec![ [6.01, 1.9], [3.2, -0.3] ]  );

    println!("{}", Polynomial::mult(&poly1, &poly2) );

    Ok(())
}
