extern crate num;

use std::io;
use std::io::{Read, Write};
use num::{BigUint, ToPrimitive};
use num::rational::Ratio;

fn factorial(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::from(1usize);
    }

    let mut a = BigUint::from(1usize);

    for i in 2..n {
        a *= i;
    }

    return a * n;
}

fn get_probability(x: usize) -> Ratio<BigUint> {
    let first_multiplier = Ratio::from(
        (factorial(90), factorial(100))
    );

    let second_multiplier = Ratio::from(
        (factorial(100 - x), factorial(90 - x))
    );
    return Ratio::from(BigUint::from(1usize)) - (first_multiplier * second_multiplier);
}

fn main() {
    println!("x  | P ");
    for num in 0..91 {
        println!("{:02} | {1:.14}", num, get_probability(num).to_f64().unwrap_or(f64::NAN))
    }
    println!("Нажмите Enter чтобы выйти.");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}
