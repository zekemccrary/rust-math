mod polynomial;
use polynomial::Polynomial;//, PolynomialStringError};
//use std::io;

fn main() {
    println!("Polynomial multiplier: for students, teachers, and everyone in between. If you're avoiding work, you're in the right place.\n");
    println!("Here's the how it should look in case you forgot:\n(3x^3 + x^2 + 26x - 5)(x^2 + 4x)(x^16 - 12x^9 + -2)... etc.");
    println!("So go ahead:\n");

    let poly = Polynomial::new(  vec![ [9.0, 6.0], [1.0, 5.0], [2.0, 4.0], [0.0, 3923.1206309], [4.0, 2.0], [13.0, 1.0], [6.0, 0.0] ]  );
    println!("{}\n", poly);

    let poly_str = "3x^3 + 33.2 - 4x + -16.998x^33.3";
    println!("{}", poly_str.parse::<Polynomial>().expect("Could not convert to polynomial"));
}
