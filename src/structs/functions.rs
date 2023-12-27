use std::fmt::{self, Display};


pub trait Function: Display {
    fn f(&self, x: f64) -> f64;

    fn to_string_sub(&self) -> String;

    fn to_string_mult(&self) -> String;

    fn to_string_parens(&self) -> String;
}



pub struct Number(pub f64);

pub struct X;

pub enum Sinusoidal<T: Function> {
    Sine(T),
    Cosine(T),
    Tangent(T),
    Arcsine(T),
    Arccosine(T),
    Arctangent(T),
}

pub struct Logarithm<A: Function, B: Function>(A, B);

pub struct SquareRoot<T: Function>(T);



pub enum OpFunc<A, B> where A: Function, B: Function {
    Add(A, B),
    Subtract(A, B),
    Multiply(A, B),
    Divide(A, B),
}

pub struct Exp<A: Function, B: Function>(A, B);




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
            Add(a, b)      => write!(f, "{a} + {b}"),

            Subtract(a, b) => write!(f, "{a} - {}", b.to_string_sub()),

            Multiply(a, b) => write!(f, "{} * {}", a.to_string_mult(), b.to_string_mult()),

            Divide(a, b)   => write!(f, "{} / {}", a.to_string_parens(), b.to_string_parens()),
        }
    }
}

impl<A, B> Display for Exp<A, B>
where A: Function, B: Function
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}^{}", self.0.to_string_parens(), self.1.to_string_parens())
    }
}





impl Function for Number {
    fn f(&self, _: f64) -> f64 {
        self.0
    }

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

    fn to_string_mult(&self) -> String {
        self.to_string()
    }

    fn to_string_parens(&self) -> String {
        self.to_string()
    }
}

impl Function for X {
    fn f(&self, x: f64) -> f64 {
        x
    }

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

    fn to_string_mult(&self) -> String {
        self.to_string()
    }

    fn to_string_parens(&self) -> String {
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

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

    fn to_string_mult(&self) -> String {
        self.to_string()
    }

    fn to_string_parens(&self) -> String {
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

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

    fn to_string_mult(&self) -> String {
        self.to_string()
    }

    fn to_string_parens(&self) -> String {
        self.to_string()
    }
}

impl<A: Function> Function for SquareRoot<A> {
    fn f(&self, x: f64) -> f64 {
        self.0.f(x).sqrt()
    }

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

    fn to_string_mult(&self) -> String {
        self.to_string()
    }

    fn to_string_parens(&self) -> String {
        self.to_string()
    }
}

impl<A: Function, B: Function> Function for OpFunc<A, B> {
    fn f(&self, x: f64) -> f64 {
        use OpFunc::*;

        match self {
            
            Add(a, b)   => a.f(x) + b.f(x),
            Subtract(a, b)  => a.f(x) - b.f(x),
            Multiply(a, b)  => a.f(x) * b.f(x),
            Divide(a, b) => a.f(x) / b.f(x),
        }
    }

    fn to_string_sub(&self) -> String {
        match self {
            OpFunc::Add(a, b)      => format!("{a} - {b}"),
            OpFunc::Subtract(a, b) => format!("{a} + {b}"),
            _              => self.to_string(),
        }
    }

    fn to_string_mult(&self) -> String {
        match self {
            OpFunc::Add(a, b)      => format!("({a} + {b})"),
            OpFunc::Subtract(a, b) => format!("({a} - {b})"),
            _                      => self.to_string(),
        }
    }

    fn to_string_parens(&self) -> String {
        format!("({self})")
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

    fn to_string_sub(&self) -> String {
        self.to_string()
    }

fn to_string_mult(&self) -> String {
    self.to_string()
}

    fn to_string_parens(&self) -> String {
        self.to_string()
    }
}