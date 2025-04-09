fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    // TODO: Replace the following with your code:
    loop {
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line); 

        if line.is_empty() {
            return;
        }

        if line.contains(&pattern_string) {
            print!("{line}");
        }
    }
}
