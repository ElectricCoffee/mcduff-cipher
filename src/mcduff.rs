const CONSONANTS: [char; 21] = [
    'B', 'C', 'D', 'F', 'G', 'H', 'J', // comments to keep rustfmt from formatting
    'K', 'L', 'M', 'N', 'P', 'Q', 'R', // a 7x3 grid is nicer than 19 + 2
    'S', 'T', 'V', 'W', 'X', 'Y', 'Z', //
];
// Digits start at 0 because the campaign takes place in the 1920s.
// That, and it's nicer to write hints that say 0-9 than 1-0.
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Case {
    Upper,
    Lower,
    Symbol,
}

/// Gets the current ascii casing of a given string, marking any non-ascii letter as `Case::Symbol`
/// Example: `get_casing(text) == vec![Upper, Lower, Lower, Lower, Lower, Symbol]`
fn get_casing<'a>(str: &'a str) -> impl Iterator<Item = Case> + 'a {
    str.chars().map(|c| match c {
        n if n.is_ascii_uppercase() => Case::Upper,
        n if n.is_ascii_lowercase() => Case::Lower,
        _ => Case::Symbol,
    })
}

/// Applies the casing in-place to a Vec<char>.
/// This assumes the casing information and the Vec<char> to be of equal length.
/// If not, the result will be truncated to the shortest one,
/// which may mess with the intended result.
fn apply_casing<Chars, Casing>(chars: Chars, casing: Casing) -> impl Iterator<Item = char>
where
    Chars: Iterator<Item = char>,
    Casing: Iterator<Item = Case>,
{
    chars.zip(casing).map(|(c, case)| match case {
        Case::Upper => c.to_ascii_uppercase(),
        Case::Lower => c.to_ascii_lowercase(),
        _ => c,
    })
}

/// Checks if a char is in the VOWELS array
fn is_vowel(char: char) -> bool {
    VOWELS.contains(&char)
}

/// Checks if a char is in the CONSONANTS array
fn is_consonant(char: char) -> bool {
    CONSONANTS.contains(&char)
}

/// Grabs the index of `val` within the iterator and returns the original iterator.
/// This is done in part because of a limitation in `Iterator::position`,
/// in that it alters the underlying iterator in-place instead of just returning the index.
fn index_and_iterator<'a, T, It>(val: T, iterator: It) -> (usize, It)
where
    T: 'a + std::cmp::Eq,
    It: Iterator<Item = &'a T> + Clone,
{
    let i = iterator.clone().position(|v| v == &val).unwrap();
    (i, iterator)
}

/// Gets the "inverse" of a char.
/// Inverse in this case means "the same position from the other end".
/// It does so individually for vowels, consonants, and digits.
/// Symbols are left alone.
/// For example, the inverse of 'a' is 'u' because 'a' is the first in "aeiou", while 'u' is the last.
/// Incidentally, this means 'i' is its own inverse.
/// The reason all three types are applied separately instead of simply reversing the alphabet is simple:
/// it leads to (somewhat) pronounceable results: "Make sure they die." becomes "Puro jako htoc xio."
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

/// Encodes and decodes a string.
/// Since the encoding is symmetric, the same function can be used for both purposes.
pub fn inverse_string(input: &str) -> String {
    let casing = get_casing(input);
    // up-casing needed because the lookup arrays all contain upper-case letters
    let input = input.to_uppercase();
    let buffer = input.chars().map(inverse_char);
    apply_casing(buffer, casing).collect::<String>()
}
