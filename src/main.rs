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

    let starting_position = parser::Fen::from_string(starting_position_file_stripped.as_str());
    let variety_test = parser::Fen::from_string(variety_test_file.as_str());

    println!("--- WIKI EXAMPLE: STARTING POSITION ---");
    println!("{starting_position_file}");
    
    println!("--- WIKI EXAMPLE: STARTING POSITION STRIPPED ---");
    println!("{starting_position_file_stripped}");
    
    println!("--- FEN: DEFAULT ---");
    println!("{}", parser::Fen::default());
    
    println!("--- FEN: STARTING POSITION ---");
    println!("{starting_position}");
    
    println!("--- FEN: VARIETY TEST ---");
    println!("{variety_test}");
    
    println!("--- ROW: EMPTY ---");
    println!("{}", parser::Row::empty());
    
    println!("--- ROW: LINE-BY-LINE ---");
    for (i, row) in parser::Fen::default().rows.iter().enumerate() {
        println!("{row}, {i}");
    }
}
