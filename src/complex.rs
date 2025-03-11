use malachite::{base::num::basic::traits::Zero, rational::Rational};
use bigdecimal::{BigDecimal, FromPrimitive};

#[derive(Clone)]
pub struct ComplexMalachite {
    a: Rational,
    b: Rational,
}

impl ComplexMalachite {
    pub fn new(x: Rational, y: Rational) -> ComplexMalachite {
        ComplexMalachite { a: x, b: y }
    }

    pub fn zero() -> ComplexMalachite {
        ComplexMalachite { a: Rational::ZERO, b: Rational::ZERO }
    }

    pub fn square(&mut self) {
        let a = &self.a;
        let a_squared = a * a;
        let b = &self.b;
        let b_squared = b * b;
        let a_b = &(a * b);

        self.a = a_squared - b_squared;
        self.b = a_b + a_b;
    }

    pub fn norm(&self) -> Rational {
        let a = &self.a;
        let a_squared = a * a;
        let b = &self.b;
        let b_squared = b * b;

        a_squared + b_squared
    }

    pub fn add(&mut self, other: &ComplexMalachite) {
        self.a += &other.a;
        self.b += &other.b;
    }
}

pub struct ComplexBigDecimal {
    a: BigDecimal,
    b: BigDecimal,
}

impl ComplexBigDecimal {
    pub fn new(x: BigDecimal, y: BigDecimal) -> ComplexBigDecimal {
        ComplexBigDecimal { a: x, b: y }
    }

    pub fn zero() -> ComplexBigDecimal {
        ComplexBigDecimal { a: BigDecimal::from_i32(0).unwrap(), b: BigDecimal::from_i32(0).unwrap() }
    }

    pub fn square(&mut self) {
        let a = &self.a;
        let a_squared = a * a;
        let b = &self.b;
        let b_squared = b * b;
        let a_b = &(a * b);

        self.a = a_squared - b_squared;
        self.b = a_b + a_b;
    }

    pub fn norm(&self) -> BigDecimal {
        let a = &self.a;
        let a_squared = a * a;
        let b = &self.b;
        let b_squared = b * b;

        a_squared + b_squared
    }

    pub fn add(&mut self, other: &ComplexBigDecimal) {
        self.a += &other.a;
        self.b += &other.b;
    }
}