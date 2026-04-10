pub mod collatz;

use std::arch::x86_64::_rdseed64_step;
use collatz::*;

fn main() {
    let mut seed = 0;
    let mut seed2 = 0;
    unsafe { _rdseed64_step(&mut seed) };
    unsafe { _rdseed64_step(&mut seed2) };

    while seed % 2 == 0 {
        unsafe { _rdseed64_step(&mut seed) };
    }

    while seed2 % 2 == 0 {
        unsafe { _rdseed64_step(&mut seed2) };
    }

    let mut generator_64 = CollatzWeyl64::new(seed);
    let mut generator128_64 = CollatzWeyl128_64::new(seed);
    let mut generator128 = CollatzWeyl128::new((seed as u128) << 64 | seed2 as u128);

    println!("64 bit: {}", generator_64.next());
    println!("128 bit: {}", generator128_64.next());
    println!("128 bit: {}", generator128.next());
}
