use std::fs;

mod parser;

fn main() {
    let starting_pos_file =
        fs::read_to_string("fen_starting_position.txt")
            .unwrap_or_default();

    let starting_pos_file_stripped =
        fs::read_to_string("fen_starting_position_stripped.txt")
            .unwrap_or_default();

    let variety_test_file =
        fs::read_to_string("fen_variety_test.txt")
            .unwrap_or_default();

    let starting_position_file = parser::Fen::from_string(starting_pos_file_stripped.as_str());
    let variety_test_file = parser::Fen::from_string(variety_test_file.as_str());

    println!("{starting_pos_file}");
    println!();
    println!("{starting_pos_file_stripped}");
    println!();
    println!("{}", parser::Fen::default());
    println!();
    println!("{starting_position_file}");
    println!();
    println!("{variety_test_file}");
    println!();
    println!("{}", parser::Row::empty());
    println!();
    for (i, row) in parser::Fen::default().rows.iter().enumerate() {
        println!("{row}, {i}");
    }
}
