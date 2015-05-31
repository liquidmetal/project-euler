// The greatest common divisor algorithm by euclid
fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }

    return a;
}

// Multiply the numbers and divide by the GCD for the LCM
fn lcm(a: u32, b: u32) -> u32 {
    return a*b / gcd(a, b);
}

fn main() {
    // Requested in the problem
    let upper: usize = 20;

    let mut prev_lcm: u32 = 1;
    for i in 1..upper {
        // Accumulating LCM works like this
        prev_lcm = lcm(i as u32, prev_lcm);
    }

    println!("The smaller multiple is {}", prev_lcm);
}
