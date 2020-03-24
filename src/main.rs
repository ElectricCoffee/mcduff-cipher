use std::iter::Iterator;

const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
const CONSONANTS: [char; 21] = [
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

enum Case {
    Upper,
    Lower,
    Symbol,
}

/// Gets the current ascii casing of a given string, marking any non-ascii letter as `Case::Symbol`
fn get_casing(str: &str) -> Vec<Case> {
    let mut vec = Vec::new();

    for c in str.chars() {
        vec.push(match c {
            n if n.is_ascii_uppercase() => Case::Upper,
            n if n.is_ascii_lowercase() => Case::Lower,
            _ => Case::Symbol,
        })
    }

    vec
}

fn apply_casing(chars: &mut Vec<char>, casing: Vec<Case>) {
    for (c, case) in chars.iter_mut().zip(casing) {
        *c = match case {
            Case::Upper => c.to_ascii_uppercase(),
            Case::Lower => c.to_ascii_lowercase(),
            _ => *c,
        }
    }
}

fn is_vowel(char: char) -> bool {
    VOWELS.contains(&char)
}

fn is_consonant(char: char) -> bool {
    CONSONANTS.contains(&char)
}

fn index_and_iterator<'a, T, It>(val: T, iterator: It) -> (usize, It)
where
    T: 'a + std::cmp::Eq,
    It: Iterator<Item = &'a T> + Clone,
{
    let i = iterator.clone().position(|v| v == &val).unwrap();
    (i, iterator)
}

fn inverse_char(char: char) -> char {
    // digits base 10
    let (index, iterator) = if char.is_digit(10) {
        index_and_iterator(char, DIGITS.iter())
    }
    // vowels
    else if is_vowel(char) {
        index_and_iterator(char, VOWELS.iter())
    }
    // consonants
    else if is_consonant(char) {
        index_and_iterator(char, CONSONANTS.iter())
    }
    // ignore the rest
    else {
        return char;
    };

    let vec = iterator.rev().collect::<Vec<_>>();
    vec[index].to_owned()
}

fn inverse_string(str: &str) -> String {
    let casing = get_casing(str);
    let str = str.to_uppercase();
    let mut buffer = Vec::new();

    for c in str.chars() {
        let c = inverse_char(c);
        buffer.push(c);
    }

    apply_casing(&mut buffer, casing);
    buffer.iter().collect::<String>()
}

fn main() {
    let test = "Ih ummoukj htoc'go zkeavth vanj, puro jako htoc yun'h ajo htop";
    let inv = inverse_string(&test);

    println!("{}", inv)
}
