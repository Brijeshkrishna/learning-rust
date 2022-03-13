pub fn scan_i32() -> i32 {

    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(..) => return -0
    };
}
pub fn scan_u32() -> usize {

    let mut input_text = String::new();
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i,
        Err(..) => return 0
    };
}



pub fn add_two_i32(a:i32,b:i32) -> i32{
    a+b
}