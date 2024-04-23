use std::fs;

mod parser;

fn main() {
    let starting_position_file =
        fs::read_to_string("fen_starting_position.txt")
            .unwrap_or_default();

    let starting_position_file_stripped =
        fs::read_to_string("fen_starting_position_stripped.txt")
            .unwrap_or_default();

    let variety_test_file =
        fs::read_to_string("fen_variety_test.txt")
            .unwrap_or_default();

    let chess_com_game_file =
        fs::read_to_string("../fen_chess.com_game.txt")
            .unwrap_or_default();

    let starting_position = parser::Fen::from_string(starting_position_file_stripped.as_str());
    let variety_test = parser::Fen::from_string(variety_test_file.as_str());
    let chess_com_game = parser::Fen::from_string(chess_com_game_file.as_str());

    println!("--- WIKI EXAMPLE: STARTING POSITION ---");
    println!("{starting_position_file}");

    println!("--- WIKI EXAMPLE: STARTING POSITION STRIPPED ---");
    println!("{starting_position_file_stripped}");

    println!("--- BY-HAND NOTATION: VARIETY TEST ---");
    println!("{variety_test_file}");

    println!("--- CHESS.COM: REAL GAME ---");
    println!("{chess_com_game_file}");

    println!("--- FEN: DEFAULT ---");
    println!("{}", parser::Fen::default());

    println!("--- FEN: STARTING POSITION ---");
    println!("{starting_position}");

    println!("--- FEN: VARIETY TEST ---");
    println!("{variety_test}");

    println!("--- FEN: CHESS.COM GAME ---");
    println!("{chess_com_game}");

    println!("--- ROW: EMPTY ---");
    println!("{}", parser::Row::empty());
}
