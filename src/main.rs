use std::io;
mod solution;
fn main() {
    loop {
        println!("Enter 0 to exit");
        println!("Enter the Problem number:");
        let mut prob_number = String::new();
        io::stdin()
            .read_line(&mut prob_number)
            .expect("Failed to read line");
        let prob_number: u16 = match prob_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid problem number");
                continue;
            }
        };
        if prob_number == 0 {
            break;
        }
        // check for solution
        solution::solution(prob_number)
    }
}
