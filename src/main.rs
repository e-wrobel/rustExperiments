use lib::{Calc, CalcTrait};

mod lib;

fn main() {
    let c = |a: i32, b: i32| a + b;
    let calc = Calc::new(10, 20, c);
    println!("{}", calc.calc());

    let c = |a: i32, b: i32| a * b;
    let mut calc = Calc::new(10, 20, c);
    calc.a = 20;
    println!("{}", calc.calc());
}
