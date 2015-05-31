fn sum_of_squares(i: u32) -> u32 {
    let mut ret = 0;
    for num in 1..(i+1) {
        ret += num * num;
    }

    return ret;
}

fn square_of_sum(i: u32) -> u32 {
    let mut sum = 0;
    for num in 1..(i+1) {
        sum += num;
    }

    return sum*sum;
}

fn main() {
    let bound: u32 = 100;

    let a = sum_of_squares(bound);
    let b = square_of_sum(bound);

    println!("The difference is {}", b-a);
}
