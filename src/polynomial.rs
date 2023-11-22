use std::fmt;
use std::string::ToString;
use std::str::FromStr;

#[derive(Debug)]
pub struct PolynomialStringError {
    pub message: String,
    pub problem: char
}

impl std::error::Error for PolynomialStringError {}

impl fmt::Display for PolynomialStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,
            "{m}\nIssue: {p}",
            m = &self.message,
            p = self.problem,
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

impl FromStr for Polynomial {
    type Err = PolynomialStringError;

    // TODO: fix too much code repetition in this function
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.replace(" ", "").replace("-", "+-");
        let mut build: Vec<[f64; 2]> = Vec::new();
        let mut termer = [0.0, 0.0];

        for term in parsed.split('+') {
            // side effect of using split and replace("-", "+-")
            if term.len() == 0 { continue; }

            // consider adding double negation support in the future
            if term == "-" {
                return Err(Self::Err {
                    message: String::from("Repeated negative sign"),
                    problem: '-',
                });
            }

            if term.contains("x") {
                if term.contains("^") {
                    let mut success = [false, false];

                    for (i, num) in term.split("x^").enumerate() {
                        if i > 1 {
                            return Err(Self::Err{
                                message: String::from("Using multiple x's in the same term"),
                                problem: 'x',
                            });
                        }

                        if let Ok(n) = num.parse::<f64>() {
                            termer[i] = n;
                            success[i] = true;
                        } else {
                            for c in term.chars() {
                                if !c.is_numeric() && c != '.'
                                { return Err(Self::Err{
                                    message: String::from("Non-numeric coefficient"),
                                    problem: c,
                                }); }
                            }

                            return Err(Self::Err{ message: String::from("Non-numeric coefficient"), problem: '?'});
                        }
                    }
                }
                else {
                    // it probably won't let me do this
                    if let Ok(n) = term.replacen("x", "", 1).parse::<f64>() {
                        termer = [n, 1.0];
                    } else {
                        for c in term.chars() {
                            if !c.is_numeric() && c != '.'
                            { return Err(Self::Err{
                                message: String::from("Non-numeric coefficient"),
                                problem: c,
                            }); }
                        }

                        // in case there wasnt a non-numeric character in there which wouldnt make sense but wtv
                        return Err(Self::Err{ message: String::from("Non-numeric coefficient"), problem: '?'});
                    }
                }
            }
            else {
                // term should be just a number if the program gets here
                if let Ok(n) = term.parse::<f64>() {
                    termer = [n, 0.0];
                } else {
                    for c in term.chars() {
                        if !c.is_numeric() && c != '.'
                        { return Err(Self::Err{
                            message: String::from("Non-numeric coefficient"),
                            problem: c,
                        }); }
                    }

                    return Err(Self::Err{ message: String::from("Non-numeric coefficient"), problem: '?'});
                }
            }

            // a coefficient of 0 is pointless
            if termer[0] == 0.0 { continue; }

            build.push( [termer[0], termer[1]] );
        }

        Ok(Polynomial{ terms: build })
    }

}