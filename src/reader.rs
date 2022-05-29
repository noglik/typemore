use std::io;

pub fn read_std_in() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    String::from(input.trim())
}
