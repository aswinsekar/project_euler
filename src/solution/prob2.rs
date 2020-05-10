// to list all even fibonacii numbers
// EFn = 4EFn-1 + EFn-2
// EF0 = 0, EF1 = 2
pub fn solution() {
    let limit = 4000000;
    let mut ef1 = 0;
    let mut ef2 = 2;
    let mut sum = ef1 + ef2;
    while ef2 < limit {
        let ef3 = 4 * ef2 + ef1;
        if ef3 > limit {
            break;
        }
        ef1 = ef2;
        ef2 = ef3;
        sum += ef2;
    }
    println!("Sum is {}", sum);
}
