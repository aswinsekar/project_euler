fn create_palidrom(mut num: u32) -> u32 {
    // creating palindrome by reversing a number
    let mut answer = num;
    while num > 0 {
        answer = (answer * 10) + (num % 10);
        num /= 10;
    }
    answer
}

pub fn solution() {
    let mut found = false;
    let mut first_half = 998;
    let mut arr: [u32; 2] = [0; 2];
    while !found {
        first_half = first_half - 1;
        let palin = create_palidrom(first_half);
        let mut i = 1000;
        while i > 99 {
            i = i - 1;
            if (palin / i) > 999 || i * i < palin {
                break;
            }
            if palin % i == 0 {
                found = true;
                arr[0] = palin / i;
                arr[1] = i;
                break;
            }
        }
    }
    println!(
        "Two numbers are {},{} and product is {}",
        arr[0],
        arr[1],
        arr[0] * arr[1]
    )
}
