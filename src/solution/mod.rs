mod prob1;
mod prob2;
mod prob3;

pub fn solution(prob_number: u16) {
    match prob_number {
        1 => prob1::solution(),
        2 => prob2::solution(),
        3 => prob3::solution(),
        _ => println!("Solution doesn't exist or problem number is out of range"),
    }
}
