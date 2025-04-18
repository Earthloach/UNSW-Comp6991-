use std::env;
use std::num::ParseIntError;

#[derive(Debug)]
struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
   }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    // TODO: complete this function!
    let mut tribonacci = vec![1_u32; 3];
    let size = size.map_err(|_| TribonacciError(error_msg))?;

    for i in 3..size {
        tribonacci.push(tribonacci[i-1] + tribonacci[i-2] + tribonacci[i-3]);
    }
    
    println!("Values: {tribonacci:?}");

    let res_sum: u32 = tribonacci.into_iter().sum();
    println!("\nSum: {res_sum}");

    Ok(())
}
