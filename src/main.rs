use std::env;
use std::io;
use std::io::Write;
use vehicle::tire::Tire;

/// Strips newlines from the end of a str
/// 
/// Example:
/// ```
/// strip_newline("Hello World!\r\n");
/// ```
pub fn strip_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}

#[test]
fn strip_newline_works(){
    assert_eq!(strip_newline("Test0\r\n\r\n"), "Test0\r\n");
    assert_eq!(strip_newline("Test1\r\n"), "Test1");
    assert_eq!(strip_newline("Test2\n"), "Test2");
    assert_eq!(strip_newline("Test3"), "Test3");
}


fn parse_tire() {
    // Read the tire size from the user
    print!("Please input a tire size to convert (275/55R20)> ");
    std::io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to get tire size!");
    if strip_newline(response.as_str()).is_empty() {
        response = String::from("275/55R20");
    }
    // Make a new tire and use it
    let tire = Tire::new(response.as_str());
    println!("Circumference: {}\"", tire.circumference());
    println!("Diameter: {}\"", tire.diameter);
    println!("Revolutions per mile: {}", tire.revs_per_mile());
    println!("Miles per revolutions: {}", tire.miles_per_rev());
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() < 2 {
        panic!("Need at least one argument!");
    }
    let first_arg = &args[1];
    match first_arg.as_str() {
        "-t" => parse_tire(),
        _ => ()
    }
}

fn main() {
    parse_args();
}
