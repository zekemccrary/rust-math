use std::fmt;
use std::string::ToString;
use std::str::FromStr;

// this should ALWAYS be a float type because we need to use NAN
type Float = f64;

#[derive(Debug)]
pub enum PolynomialStringError {
    EmptyStringError,
    ParseError(char),
    ParenthesesError(bool),
}

impl std::error::Error for PolynomialStringError {}

impl fmt::Display for PolynomialStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use PolynomialStringError::*;
        match self {
            EmptyStringError    => write!(formatter, "No polynomial found in string"                              ),
            ParseError(c)    => write!(formatter, "Failure to parse, encountered character: {}", c      ),
            ParenthesesError(b) => write!(formatter, "Illegal {} parentheses", if *b { "open" } else { "close" }  ),
        }
    }
}

#[derive(Debug)]
pub struct Polynomial {
    terms: Vec<[Float; 2]>,
}

impl Polynomial {
    pub const fn new(v: Vec<[Float; 2]>) -> Polynomial {
        Polynomial { terms: v }
    }

    pub fn simplify(&self) -> Polynomial {
        const NINEK: f64 = 1000000000.0_f64;

        let mut build: Vec<[Float; 2]> = Vec::with_capacity(self.terms.len());

        'outer: for term in self.terms.iter() {
            if term[0] == 0.0 { continue; }

            for (i, build_term) in build.iter().enumerate() {
                if term[1] == build_term[1] {
                    // prevent floating point addition errors
                    let num = term[0] + build_term[0];
                    let round = num.floor();

                    build[i] = [round + ((num - round) * NINEK).round() / NINEK, term[1]];
                    continue 'outer;
                }
            }
        
            build.push(*term);
        }

        Polynomial{ terms: build }
    }

    pub fn organize(&self) -> Polynomial {
        let mut build: Vec<[Float; 2]> = Vec::with_capacity(self.terms.len());
        let mut index: usize;
        
        // dummy value bc of insert
        build.push([0.0, f64::INFINITY]);

        for term in self.terms.iter() {
            index = 0;

            for (i, build_term) in build.iter().enumerate() {
                index = i;
                if build_term[1] > term[1] { break; }
            }

            build.insert(index, term.clone());
        }

        let _ = build.pop();

        Polynomial{ terms: build }
    }
}


impl fmt::Display for Polynomial {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let num_terms = self.terms.len() - 1;
        let mut build = String::with_capacity(num_terms * 5 + 4); // 5(len - 1) + 4 = 5(len) - 1

        for (i, [coeff, exp]) in self.terms.iter().enumerate() {
            // please look past the ugly match statement used to trick the compiler into letting me use one
            match *coeff {
                // 0 * anything = 0
                n if n == 0.0 => continue,

                // 1 * x = x
                n if n == 1.0 =>
                match *exp {
                    n if n == 0.0 => build.push('1'), // 1x^0 = 1
                    n if n == 1.0 => build.push('x'), // 1x^1 = x
                    _             => build.push_str( &format!("x^{val}", val = &exp.to_string()) ),
                },

                n if n == -1.0 =>
                match *exp {
                    n if n == 0.0 => build.push_str("-1"),
                    n if n == 1.0 => build.push_str("-x"),
                    _             => build.push_str( &format!("-x^{val}", val = &exp.to_string()) ),
                }

                _             =>
                match *exp {
                    n if n == 0.0 => build.push_str( &coeff.to_string() ), // x^0 = 1
                    n if n == 1.0 => build.push_str( &format!("{te}x", te = &coeff.to_string()) ), // x^1 = x
                    _             => build.push_str( &format!("{te}x^{ex}", te = &coeff.to_string(), ex = &exp.to_string()) ),
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum Mode {
            Number(char),
            X,
            Operator(bool),
            Arrow,
        }
        
        impl Mode {
            // TODO: change to only use one return keyword
            const fn new(c: char) -> Result<Self, PolynomialStringError> {
                return match c {
                    '^'                     => Ok(  Self::Arrow                                      ),
                    'x' | 'X'               => Ok(  Self::X                                          ),
                    '+' | '-'               => Ok(  Self::Operator(if c == '+' {true} else {false})  ),
                    '.'                     => Ok(  Self::Number(c)                                  ),
                    _ if c.is_ascii_digit() => Ok(  Self::Number(c)                                  ),
        
                    _                       => Err(PolynomialStringError::ParseError(c)),
                }
            }
        
            // TODO: change to only use one return keyword
            fn char(&self) -> char {
                use Mode::*;
                match self {
                    Number(c)   => *c,
                    Operator(b) => if *b {'+'} else {'-'},
                    X           => 'x',
                    Arrow       => '^',
                }
            }
        }
        

        struct Expected<'a> { modes: &'a [Mode] }
        
        impl Expected<'_> {
            fn matches(&self, mode: &Mode) -> Result<Self, PolynomialStringError> {
                use std::mem::discriminant;
        
                for m in self.modes {
                    if discriminant(m) != discriminant(mode) { continue; }
        
                    // TODO: change to only use one return keyword
                    use Mode::*;
                    return match mode {
                        Number(_)   => Ok(Expected{ modes: &[ Number(0 as char), Operator(false), X, Arrow ] }),
                        X           => Ok(Expected{ modes: &[ Operator(false), Arrow ] }),
                        Operator(_) => Ok(Expected{ modes: &[ Number(0 as char), X ] }),
                        Arrow       => Ok(Expected{ modes: &[ Number(0 as char), Operator(false), Arrow ] }),
                    }
                }
        
                Err(PolynomialStringError::ParseError(mode.char()))
            }
        }


        fn parse_float(fstr: &mut String, err: char) -> Result<Float, PolynomialStringError> {

            if let Ok(n) = fstr.parse::<Float>() {
                *fstr = String::new();
                return Ok(n);
            }
        
            Err(PolynomialStringError::ParseError(err))
        }


        let input = format!("{}+", s);

        let mut parsed: Vec<[Float; 2]> = Vec::new();
        let mut expected: Expected = Expected { modes: &[Mode::Number(0 as char), Mode::X] };
        let mut char_mode: Mode;
        let mut arr_build = [Float::NAN; 2];
        let mut num_build = String::new();
        let mut sign_mode = true;

        for c in input.chars() {
            if c == ' ' { continue; }

            char_mode = Mode::new(c)?;
            expected = expected.matches(&char_mode)?;

            match char_mode {
                Mode::Number(c) => num_build.push(c),

                /*
                    Cases to handle:
                    1. no coefficient was found (num_build is empty) ->
                        set coefficient to 1
                    2. coefficient was found ->
                        parse num_build to coefficient
                        set exponent to 1 (marker that an X was found in this term)
                */
                Mode::X => {
                    // 1.
                    if num_build.len() == 0 { num_build.push('1'); }

                    // 2.
                    if arr_build[0].is_nan() {
                        arr_build[0] = parse_float(&mut num_build, 'x')?;
                        arr_build[1] = 1.0;
                    }
                    else { return Err(PolynomialStringError::ParseError('x')); }
                },

                /*
                    Cases to handle:
                    1. no X was found ->
                        parse num_build to coefficient
                        apply coefficient sign
                        set exponent to 0
                        reset variables
                    2. X was found but no arrow ->
                        reset variables (everything else has already been done)
                    3. X was found and arrow was found ->
                        1. this operator is for the exponent ->
                            set sign_mode
                        2. this operator ends the term ->
                            parse num_build to exponent
                            apply exponent sign
                            reset variables
                */
                Mode::Operator(b) => {
                    let c = if b {'+'} else {'-'};

                    // 1.
                    if arr_build[0].is_nan() {
                        arr_build[0] = parse_float(&mut num_build, c)? * (if sign_mode {1.0} else {-1.0});
                        arr_build[1] = 0.0;
                    }
                    // 3.
                    else if arr_build[1].is_nan() {
                        // 3.1.
                        if num_build.len() == 0 {
                            sign_mode = b;
                            continue;
                        }
                        // 3.2.
                        else {
                            arr_build[1] = parse_float(&mut num_build, c)?;
                            if !sign_mode { arr_build[1] *= -1.0; }
                        }
                    }
                    // 2. (in a way)
                    else if arr_build[1] != 1.0 {
                        return Err(PolynomialStringError::ParseError(c));
                    }

                    sign_mode = b;

                    // reset for the next term
                    parsed.push(arr_build);
                    arr_build = [Float::NAN; 2];
                },

                /*
                    Cases to handle:
                    1. X has been found ->
                        set exponent to NAN (marker that an arrow was found)
                        apply coefficient sign
                */
                Mode::Arrow => {
                    // 1.
                    if !arr_build[0].is_nan() && arr_build[1] == 1.0 {
                        arr_build[1] = Float::NAN;

                        if !sign_mode {
                            arr_build[0] *= -1.0;
                            sign_mode = true;
                        }
                    }
                    else {
                        return Err(PolynomialStringError::ParseError('^'));
                    }
                },
            }

        }

        Ok(Polynomial{ terms: parsed })
    }

}
