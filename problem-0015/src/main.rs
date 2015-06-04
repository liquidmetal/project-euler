fn binomial(n: u32, r: u32) -> u64 {
    let mut k = r;
    if n-r < k {
        k = n-r;
    }
    let k = k as u64;

    let mut ans = 1 as u64;
    let mut j = 1 as u64;
    let mut n = n as u64;
    loop {
        if n % j == 0 {
            ans *= (n/j);
        } else {
            if ans % j == 0 {
                ans /= j;
                ans *= n;
            } else {
                ans *= n;
                ans /= j;
            }
        }
        n-=1;
        j+=1;

        if j>k {
            break;
        }
    }

    return ans as u64;
}

fn main() {
    let size = 20;
    let total_paths = size * 2;
    
    // Calculate 40C20 (from 40 choose 20)
    println!("{}", binomial(40, 20));
}
