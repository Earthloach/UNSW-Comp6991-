use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Unable to flush stdout");
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Unable to read the line");     

    let name = name.trim();

    if !name.is_empty() {
        println!("Hello, {}, nice to meet you!", name);
    } else {
        println!("No name entered :(, goodbye.");
    }
}
