use std::fmt::{self, Display};


#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
}


pub trait Function: Display {
    fn f(&self, x: f64) -> f64;

    fn to_string_parens(&self, op: u8) -> String;
}


#[derive(Debug)]
pub struct Number(pub f64);

#[derive(Debug)]
pub struct X;

#[derive(Debug)]
pub enum Sinusoidal<T: Function> {
    Sine(T),
    Cosine(T),
    Tangent(T),
    Arcsine(T),
    Arccosine(T),
    Arctangent(T),
}

#[derive(Debug)]
pub struct Logarithm<A: Function, B: Function>(pub A, pub B);

#[derive(Debug)]
pub struct SquareRoot<T: Function>(pub T);

#[derive(Debug)]
pub enum OpFunc<A, B> where A: Function, B: Function {
    Add(A, B),
    Subtract(A, B),
    Multiply(A, B),
    Divide(A, B),
}

#[derive(Debug)]
pub struct Exp<A: Function, B: Function>(pub A, pub B);

pub struct MathFunction {
    terms: Vec<Box<dyn Function>>,
    operators: Vec<Operator>,
}

impl MathFunction {
    pub fn new(f: Box<dyn Function>) -> MathFunction {
        MathFunction { 
            terms: vec![f],
            operators: Vec::<Operator>::new(),
        }
    }

    pub fn push(&mut self, op: Operator, term: Box<dyn Function>) {
        self.terms.push(term);
        self.operators.push(op);
    }
}



impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for X {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x")
    }
}

impl<T: Function> Display for Sinusoidal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Sinusoidal::*;
        
        match self {
            Sine(n)       => write!(f, "sin({})", n),
            Cosine(n)     => write!(f, "cos({})", n),
            Tangent(n)    => write!(f, "tan({})", n),
            Arcsine(n)    => write!(f, "arcsin({})", n),
            Arccosine(n)  => write!(f, "arccos({})", n),
            Arctangent(n) => write!(f, "arctan({})", n),
        }
    }
}

impl<A, B> Display for Logarithm<A, B>
where A: Function, B: Function
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "log[{}]({})", self.0, self.1)
    }
}

impl<T: Function> Display for SquareRoot<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sqrt({})", self.0)
    }
}

impl<A, B> Display for OpFunc<A, B>
where A: Function, B: Function
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use OpFunc::*;

        match self {
            Add(a, b)      => write!(f, "{a} + {b}" ),

            Subtract(a, b) => write!(f, "{a} - {}", b.to_string_parens(Operator::Minus as u8) ),

            Multiply(a, b) => write!(f, "{} * {}", a.to_string_parens(Operator::Asterisk as u8), b.to_string_parens(Operator::Asterisk as u8) ),

            Divide(a, b)   => write!(f, "{} / {}", a.to_string_parens(Operator::Slash as u8), b.to_string_parens(Operator::Slash as u8) ),
        }
    }
}

impl<A, B> Display for Exp<A, B>
where A: Function, B: Function
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}^{}", self.0.to_string_parens(Operator::Caret as u8), self.1.to_string_parens(Operator::Caret as u8))
    }
}

impl Display for MathFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* MathFunction manually checks that terms.len() - 1 == operators.len() at creation */

        let mut division_lookahead = false;
        let mut iterator = self.terms.iter();
        let mut build;

        if let Some(t) = iterator.next()
        { build = t.to_string(); }
        else
        { return fmt::Result::Err(fmt::Error); }

        for (i, term) in iterator.enumerate() {
            let op = self.operators[i];

            if i < self.operators.len() {
                division_lookahead = self.operators[i + 1] == Operator::Slash;
            }

            match self.operators[i] {
                Operator::Plus     => build.push_str(" + "),
                Operator::Minus    => build.push_str(" - "),
                Operator::Asterisk => build.push_str(" * "),
                Operator::Slash    => build.push_str(" / "),
                Operator::Caret    => build.push_str("[unrecognized character]"),
            }

            build.push_str(
                &({
                    let s;
                    if division_lookahead { s = format!("({})", term.to_string_parens(op as u8))}
                    else                  { s = term.to_string_parens(op as u8) }

                    s
                })
            );
        }

        write!(f, "{build}")
    }
}





impl Function for Number {
    fn f(&self, _: f64) -> f64 {
        self.0
    }

    fn to_string_parens(&self, _: u8) -> String {
        self.to_string()
    }
}

impl Function for X {
    fn f(&self, x: f64) -> f64 {
        x
    }

    fn to_string_parens(&self, _: u8) -> String {
        self.to_string()
    }
}

impl<A: Function> Function for Sinusoidal<A> {
    fn f(&self, x: f64) -> f64 {
        use Sinusoidal::*;

        match self {
            Sine(n)       => n.f(x).sin(),
            Cosine(n)     => n.f(x).cos(),
            Tangent(n)    => n.f(x).tan(),
            Arcsine(n)    => n.f(x).asin(),
            Arccosine(n)  => n.f(x).acos(),
            Arctangent(n) => n.f(x).atan(),
        }
    }

    fn to_string_parens(&self, _: u8) -> String {
        self.to_string()
    }
}

impl<A: Function, B: Function> Function for Logarithm<A, B> {
    fn f(&self, x: f64) -> f64 {
        let b = self.0.f(x);
        let a = self.1.f(x);
        
        match b as i32 {
            10 => a.log10(),
            
            2 => a.log2(),

            _ if b == std::f64::consts::E => a.ln(),

            _ => a.log(b),
        }
    }

    fn to_string_parens(&self, _: u8) -> String {
        self.to_string()
    }
}

impl<A: Function> Function for SquareRoot<A> {
    fn f(&self, x: f64) -> f64 {
        self.0.f(x).sqrt()
    }

    fn to_string_parens(&self, _: u8) -> String {
        self.to_string()
    }
}

impl<A: Function, B: Function> Function for OpFunc<A, B> {
    fn f(&self, x: f64) -> f64 {
        use OpFunc::*;

        match self {
            Add(a, b)       => a.f(x) + b.f(x),
            Subtract(a, b)  => a.f(x) - b.f(x),
            Multiply(a, b)  => a.f(x) * b.f(x),
            Divide(a, b)    => a.f(x) / b.f(x),
        }
    }

    fn to_string_parens(&self, op: u8) -> String {
        match op {
            3u8 | 4u8 => format!("({self})"),

            1u8 => match self {
                        OpFunc::Add(a, b)      => format!("{a} - {b}"),
                        OpFunc::Subtract(a, b) => format!("{a} + {b}"),
                        _                              => self.to_string(),
                    },

            2u8 => match self {
                        OpFunc::Add(a, b)      => format!("({a} + {b})"),
                        OpFunc::Subtract(a, b) => format!("({a} - {b})"),
                        _                      => self.to_string(),
                    },

            _       => self.to_string(),
        }
    }
}

impl<A: Function, B: Function> Function for Exp<A, B> {
    fn f(&self, x: f64) -> f64 {
        let one = self.0.f(x);
        let two = self.1.f(x);
        let two_as_i32 = two as i32;

        if two == Into::<f64>::into(two_as_i32)
        { return one.powi(two_as_i32); }
        
        one.powf(two)
    }

    fn to_string_parens(&self, op: u8) -> String {
        match op {
            4u8 => format!("({self})"),
            _   => self.to_string(),
        }
    }
}

impl Function for MathFunction {
    fn f(&self, x: f64) -> f64 {
        use Operator::*;

        let mut termvec: Vec<f64> = Vec::with_capacity(self.terms.len());

        // desugared for loop becaase Asterisk and Slash cases must skip an element
        let mut opiter = self.operators.iter().enumerate();
        while let Some((i, op)) = opiter.next() {
            match op {
                // add together on second pass
                Plus | Minus  => termvec.push(self.terms[i].f(x)),

                // multiplication and division must be handled first because of order of operations conventions
                Asterisk      => {
                                    termvec.push(self.terms[i].f(x) * self.terms[i+1].f(x));
                                    opiter.nth(0);
                                },

                Slash         => {
                                    termvec.push(self.terms[i].f(x) / self.terms[i+1].f(x));
                                    opiter.nth(0);
                                },

                _             => (),
            }
        }

        let mut termiter = termvec.iter();

        let mut build = {
            if let Some(b) = termiter.next() 
            { *b }
            else 
            { 0.0 }
        };

        for (i, op) in self.operators.iter().enumerate() {

            match op {
                Plus => {
                    if let Some(term) = termiter.next() {
                        build += term;
                    } else {
                        break;
                    }
                },
                Minus => {
                    if let Some(term) = termiter.next() {
                        build -= term;
                    } else {
                        break;
                    }
                },
                _ => (),
            }
        }



        build
    }

    fn to_string_parens(&self, op: u8) -> String {
        match op {
            0u8  => self.to_string(),
            1u8 => {
                let s = self.to_string();
                let mut nu = String::with_capacity(s.len());

                for c in s.chars() {
                    nu.push(
                        match c {
                            '+' => '-',
                            '-' => '+',
                            _   => c,
                        }
                    );
                }

                nu
            },
            2u8 | 3u8 => format!("({self})"),
            _    => self.to_string(),
        }
    }
}