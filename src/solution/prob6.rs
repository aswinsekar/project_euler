fn sqr_sum_of_n_numbers(n: u64) -> u64 {
    ((n * (n + 1)) / 2).pow(2)
}
fn sum_of_square_n_numbers(n: u64) -> u64 {
    (n * (n + 1) * ((2 * n) + 1)) / 6
}
pub fn solution() {
    println!(
        "{}",
        sqr_sum_of_n_numbers(100) - sum_of_square_n_numbers(100)
    )
}
