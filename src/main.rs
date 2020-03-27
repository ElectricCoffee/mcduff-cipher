mod mcduff;
use mcduff::inverse_string;
use std::env;

fn invert_and_print(str: &str) {
    let inv = inverse_string(str);
    println!("Original: {}", str);
    println!("Ciphered: {}", inv);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Please enter some text to convert.");
        return;
    }

    if args.len() == 2 {
        invert_and_print(&args[1]);
        return;
    }

    for (i, arg) in args[1..].iter().enumerate() {
        println!("--- String {} ---", i + 1);
        invert_and_print(&arg);
    }
}
