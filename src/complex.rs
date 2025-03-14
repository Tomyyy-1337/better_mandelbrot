#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    a: T,
    b: T,
}

impl<T> Complex<T> {
    pub fn new(a: T, b: T) -> Complex<T> {
        Complex { a, b }
    }
}

impl<T> Complex<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    pub fn norm(&self) -> T {
        self.a * self.a + self.b * self.b
    }
}

impl<T> std::ops::AddAssign for Complex<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, other: Complex<T>) {
        self.a += other.a;
        self.b += other.b;
    }
}

impl<T> std::ops::MulAssign for Complex<T>
where
    T: Copy 
        + std::ops::Add<Output = T> 
        + std::ops::Mul<Output = T> 
        + std::ops::Sub<Output = T>,
{
    fn mul_assign(&mut self, other: Complex<T>) {
        let a = self.a;
        let b = self.b;
        self.a = a * other.a - b * other.b;
        self.b = a * other.b + b * other.a;
    }
}
