use std::fs;

mod parser;

fn main() {
    let starting_pos_file =
        fs::read_to_string("fen_starting_position.txt")
            .unwrap_or_default();

    let starting_pos_file_stripped =
        fs::read_to_string("fen_starting_position_stripped.txt")
            .unwrap_or_default();

    let custom_fen = parser::Fen::from_string(starting_pos_file_stripped.as_str());

    println!("{starting_pos_file}");
    println!();
    println!("{starting_pos_file_stripped}");
    println!();
    println!("{}", parser::Fen::default());
    println!();
    println!("{custom_fen}");
}
