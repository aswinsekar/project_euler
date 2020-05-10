mod prob1;

pub fn solution(prob_number: u16) {
    match prob_number {
        1 => prob1::solution(),
        _ => println!("Solution doesn't exist or problem number is out of range"),
    }
}
