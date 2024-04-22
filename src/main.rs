use std::fs;

mod parser;

fn main() {
    let starting_pos_file =
        fs::read_to_string("fen_starting_position.txt")
            .unwrap_or_default();

    let starting_pos_file_simplified =
        fs::read_to_string("fen_starting_position_simplified.txt")
            .unwrap_or_default();

    println!("{starting_pos_file}");
    println!("{starting_pos_file_simplified}");
    println!("{}", parser::Fen::default());

    let custom_fen = parser::Fen::from_string(starting_pos_file_simplified.as_str());

    println!("{custom_fen}");
}
