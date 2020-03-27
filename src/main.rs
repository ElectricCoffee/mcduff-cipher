mod mcduff;
use mcduff::inverse_string;

fn main() {
    let test = "Grumpy wizard makes toxic brew for the evil Queen and Jack.";
    let inv = inverse_string(&test);

    println!("Original: {}", test);
    println!("Inverted: {}", inv);

    assert_eq!(
        "Vkapmc fibukx puroj hediy zkof wek hto ogiq Laoon unx Suyr.".to_owned(),
        inv
    );
}
