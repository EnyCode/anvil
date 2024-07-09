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

pub fn prettify_pascal_case(string: String) -> String {
    let mut formatted = String::new();

    for c in string.chars() {
        if c.is_uppercase() && formatted.len() > 0 {
            formatted.push(' ');
        }
        formatted.push(c);
    }

    formatted
}
