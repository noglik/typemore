use std::io;

pub fn read_std_in() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(error) => println!("error: {}", error),
    }
    return input;
}
