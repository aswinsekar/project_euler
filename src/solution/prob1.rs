pub fn solution() {
    let mut sum = 0;
    for n in 3..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n
        }
    }
    println!("Sum is {}", sum);
}
