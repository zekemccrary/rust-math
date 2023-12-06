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

    let poly = Polynomial::new(vec![[2.5, 2.0], [4.6, 3.0], [3.0, 2.0], [1.0, 0.3], [2.1, 3.0], [0.0, -13.0], [0.239, -6.2], [1.1, -6.3], [0.5, -6.2]]);
    let simplified = poly.simplify();
    let organized = poly.organize();
    let both = simplified.organize();
    println!("");
    println!("raw: {}", poly);
    println!("simplified: {}", simplified);
    println!("organized: {}", organized);
    println!("both: {}", both);

    Ok(())
}
