enum Color {
    Red, 
    Green,
    Blue,
    Yellow,
}

fn main() {
    let green = Color::Green;
    let blue = Color::Blue;
    let red = Color::Red;
    let yellow = Color::Yellow;

    println!("Red: {}", color_to_str(&red));
    println!("Green: {}", color_to_str(&green));
    println!("Blue: {}", color_to_str(&blue));
    println!("Yellow: {}", color_to_str(&yellow));

    find_maybe_number(Some(42));
    find_maybe_number(None);
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// Optionもenumなのでmatchでパターン列挙可能
fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found {}", number), 
        None => println!("Nothing found"), 
    }
}

fn color_to_str(color: &Color) -> &str {
    // Red #FF0000
    // Green #00FF00
    // Blue #0000FF
    // Yellow #FFFF00

    // matchはすべてのパターン列挙が必要
    match color {
        Color::Red => "#FF0000", 
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
        Color::Yellow => "#FFFF00",
    }
}
