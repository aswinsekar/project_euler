pub fn solution() {
    let mut number = 600_851_475_143u64;
    let mut max_prime = 0;
    while number % 2 == 0 {
        max_prime = 2;
        number >>= 1;
    }
    let mut i = 3;
    while i <= (number as f64).sqrt() as u64 {
        while number % i == 0 {
            max_prime = i;
            number = number / i;
        }
        i += 2
    }
    if number > 2 {
        max_prime = number;
    }
    println!("The largest prime number {}", max_prime)
}
