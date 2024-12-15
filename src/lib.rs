pub struct Calc {
    pub a: i32,
    b: i32,
    calc_fun: fn(a: i32, b: i32) -> i32,
}

pub trait CalcTrait {
    fn new(a: i32, b: i32, calc: fn(a: i32, b: i32) -> i32) -> Self;
    fn calc(self) -> i32;
}

impl CalcTrait for Calc {
    fn new(a: i32, b: i32, calc: fn(a: i32, b: i32) -> i32) -> Calc {
        Calc { a, b, calc_fun: calc }
    }

    fn calc(self) -> i32 {
        (self.calc_fun)(self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::{Calc, CalcTrait};

    #[test]
    fn test_addition() {
        let c = |a: i32, b: i32| a + b;
        let calc = Calc::new(10, 20, c);
        assert_eq!(calc.calc(), 30);
    }

    #[test]
    fn test_multiplication() {
        let c = |a: i32, b: i32| a * b;
        let calc = Calc::new(10, 20, c);
        assert_eq!(calc.calc(), 200);
    }

    #[test]
    fn test_subtraction() {
        let c = |a: i32, b: i32| a - b;
        let calc = Calc::new(20, 10, c);
        assert_eq!(calc.calc(), 10);
    }

    #[test]
    fn test_division() {
        let c = |a: i32, b: i32| a / b;
        let calc = Calc::new(20, 10, c);
        assert_eq!(calc.calc(), 2);
    }
}