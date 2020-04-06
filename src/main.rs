mod mcduff;
use mcduff::inverse_string;
use std::{env, io::{self, Write}, error::Error};

type DynamicResult<T> = Result<T, Box<dyn Error>>;

fn invert_and_print(str: &str) {
    let inv = inverse_string(str);
    println!("Original: {}", str);
    println!("Ciphered: {}", inv);
}

fn repl() -> DynamicResult<()> {
    println!("Welcome to the McDuff Cipher repl.\n\
    Please enter the message you want to encrypt or decrypt.");
    let mut buffer = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let inverse = inverse_string(&buffer);
        println!("{}", inverse);
        buffer.clear();
    }
}

fn main() -> DynamicResult<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        repl()?;
    }

    if args.len() == 2 {
        invert_and_print(&args[1]);
        return Ok(());
    }

    for (i, arg) in args[1..].iter().enumerate() {
        println!("--- String {} ---", i + 1);
        invert_and_print(&arg);
    }

    Ok(())
}
