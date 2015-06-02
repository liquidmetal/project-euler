fn next_term(n: u64) -> u64 {
    if n % 2 == 0 {
        return n/2;
    } else {
        return 3*n+1;
    }
}

fn main() {
    let mut current_start = 1000000;
    let mut max_iterations = 0;
    let mut max_number = 0;
    loop {
        current_start -= 1;
        let mut i = current_start;
        let mut iterations = 0;
        while i > 1 {
            i = next_term(i);
            iterations += 1;
        }

        if iterations > max_iterations {
            max_iterations = iterations;
            max_number = current_start;
        }

        if current_start == 1 {
            break;
        }
    }

    println!("Max iterations = {}", max_iterations);
    println!("Max number = {}", max_number);
}
