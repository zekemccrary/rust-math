use std::fmt;
use std::string::ToString;
use std::str::FromStr;

#[derive(Debug)]
pub struct PolynomialStringError {
    pub message: String,
    pub polynomial: String,
    pub index: usize,
}

impl std::error::Error for PolynomialStringError {}

impl fmt::Display for PolynomialStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
            "{m}\n{p}\n{s}^",
            m = &self.message,
            p = &self.polynomial,
            s = &( String::from_utf8(vec![32; self.index]).expect("Could not convert vec to string") ),
        )
    }
}

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<[f64; 2]>,
}

impl Polynomial {
    pub const fn new(v: Vec<[f64; 2]>) -> Polynomial {
        Polynomial { terms: v }
    }
}


impl fmt::Display for Polynomial {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let num_terms = self.terms.len() - 1;
        let mut build = String::with_capacity(num_terms * 5 + 4); // 5(len - 1) + 4 = 5(len) - 1

        for (i, [coeff, exp]) in self.terms.iter().enumerate() {
            // somebody please explain to me why i have to manually dereference
            match *coeff as i32 {
                // 0 * anything = 0
                0 => continue,

                // 1 * x = x
                1 => match *exp as i32 {
                    0 => build.push('1'), // 1x^0 = 1
                    1 => build.push('x'), // 1x^1 = x
                    _ => build.push_str( &format!("x^{val}", val = &exp.to_string()) ),
                },

                _ => match *exp as i32 {
                    0 => build.push_str( &coeff.to_string() ), // x^0 = 1
                    1 => build.push_str( &format!("{te}x", te = &coeff.to_string()) ), // x^1 = x
                    _ => build.push_str( &format!("{te}x^{ex}", te = &coeff.to_string(), ex = &exp.to_string()) ),
                },
            }
            
            if i != num_terms {
                build.push_str(" + ");
            }
        }

        write!(formatter, "{}", build)
    }
}