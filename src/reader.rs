use std::io;

pub fn read_std_in() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    return input.trim().to_string();
}
