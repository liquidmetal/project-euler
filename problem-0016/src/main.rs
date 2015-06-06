extern crate num;

use num::bigint::BigInt;
use num::FromPrimitive;
use num::pow;

fn main() {
    let n: BigInt = FromPrimitive::from_u32(2).unwrap();
    let p = pow(n, 1000);

    let s: String = p.to_string();

    let chars = s.chars();
    let mut sum = 0;
    for i in chars {
        let ascii = i as u32;
        sum += ascii-48;
    }
    println!("Total sum = {}", sum);
}
