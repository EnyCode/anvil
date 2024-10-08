/// converts an integer to a roman numeral with one space prior.
/// the number 1 will return an empty string.
/// ```
/// assert_eq!(to_roman_numerals(1), String::from(""));
/// assert_eq!(to_roman_numerals(2), String::from(" II"));
/// assert_eq!(to_roman_numerals(3), String::from(" III"));
/// ```
pub fn to_roman_numerals(n: u32) -> String {
    let default = format!(" {n}");

    match n {
        1 => "",
        2 => " II",
        3 => " III",
        4 => " IV",
        5 => " V",
        _ => &default,
    }
    .to_string()
}

/// converts pascal case to title-cased words, with some exceptions.
/// ```
/// assert_eq!(prettify_pascal_case("BookAndQuill"), String::from("Book and Quill"));
/// assert_eq!(prettify_pascal_case("LuckOfTheSea"), String::from("Luck of the Sea"));
/// ```
pub fn prettify_pascal_case(string: String) -> String {
    let mut formatted = String::new();

    for c in string.chars() {
        if c.is_uppercase() && formatted.len() > 0 {
            formatted.push(' ');
        }
        formatted.push(c);
    }

    formatted
        .replace(" And ", " and ")
        .replace(" The ", " the ")
        .replace(" Of ", " of ")
        .replace(" On ", " on ")
        .replace(" A ", " a ")
}
