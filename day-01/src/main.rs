fn main() {
    let digits: Vec<u8> = b"1212"
        .into_iter()
        .map(|c| c - 48)
        .collect();
    let length = digits.len();
    let mut sum = 0;
    for i in 0..length {
        /*
        if digits[i] == digits[(i + 1) % length] {
            sum += i32::from(digits[i]);
        }
        */
        if digits[i] == digits[(i + length / 2) % length] {
            sum += i32::from(digits[i]);
        }
    }
    println!("{}", sum);
}
