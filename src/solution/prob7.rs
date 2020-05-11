// store all the found prime to check for future prime

fn is_prime(n: u64, prime: &Vec<u64>) -> bool {
    let mut is_prime: bool = true;
    for i in prime.iter() {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            is_prime = false;
        }
    }
    is_prime
}

pub fn solution() {
    let mut prime = Vec::new();
    prime.push(2);
    let mut i = 3;
    while prime.len() <= 10001 {
        if is_prime(i, &prime) {
            prime.push(i);
        }
        i = i + 1;
    }
    println!("10001st prime is {}", prime[10000]) // since vec is zero based index
}
