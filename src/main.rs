mod mcduff;
use mcduff::inverse_string;

fn main() {
    let test = "this is a test";
    let inv = inverse_string(&test);

    println!("{}", inv)
}
