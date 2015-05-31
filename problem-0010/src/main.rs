fn is_prime(i: u32) -> bool {
    if i==2 {
        return true;
    }
    if i%2==0 {
        return false;
    }
    for factor in 3..((i as f64).sqrt() as u32 +1) {
        if i % factor == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let bound = 2000000;
    let mut sum: u64 = 0;
    for i in 2..bound {
        if is_prime(i) {
            sum += i as u64;
        }
    }
    println!("The sum is {}", sum);
}
