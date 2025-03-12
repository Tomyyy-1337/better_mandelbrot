use f256::f256;

pub struct ComplexF256 {
    a: f256,
    b: f256,
}

impl ComplexF256 {
    pub fn new(x: f256, y: f256) -> ComplexF256 {
        ComplexF256 { a: x, b: y }
    }

    pub fn zero() -> ComplexF256 {
        ComplexF256 {
            a: f256::from(0),
            b: f256::from(0),
        }
    }

    pub fn square(&mut self) {
        let a_squared = self.a * self.a;
        let b_squared = self.b * self.b;
        let a_b = self.a * self.b;

        self.a = a_squared - b_squared;
        self.b = a_b + a_b;
    }

    pub fn norm(&self) -> f256 {
        self.a * self.a + self.b * self.b
    }

    pub fn add(&mut self, other: &ComplexF256) {
        self.a += &other.a;
        self.b += &other.b;
    }
}
