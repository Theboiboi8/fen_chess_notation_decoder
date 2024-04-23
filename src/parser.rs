use std::fmt::{Display, Formatter};

pub struct Fen {
	/// A vector of information for every row starting at index 0 up to index 7,
	/// where the index maps to chessboard rows 1-8 starting at row 1 for index 0
	pub rows: Vec<Row>
}

#[derive(Clone)]
pub struct Row {
	/// A vector of information for every piece starting at index 0 up to index 7,
	/// where the index maps to chessboard columns A-H starting at column A for index 0
	pub pieces: Vec<Piece>
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piece {
	pub piece_type: PieceType,
	pub color: PieceColor
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King,
	#[default]
	Empty
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum PieceColor {
	White,
	Black,
	#[default]
	Empty
}

impl Fen {
	/// Takes a FEN notation string and converts it to a [`Fen`].
	/// 
	/// The rows in the FEN notation are separated by a `/`,
	/// and as of right now king and queen status is not handled,
	/// and neither is move count and side playing  
	pub fn from_string(input: &str) -> Self {
		// TODO: Implement number notation for empty spaces
		let mut rows = Vec::<Row>::with_capacity(8);
		//let input = input.to_string();
		
		for input_row in input.split('/') {
			let mut row = Row::empty();
			
			for (i, char) in input_row.chars().enumerate() {
				assert!(i <= 8, "More than 8 characters provided in input row {input_row}");
				
				match char {
					'p' => row.pieces[i] = Piece::white_piece(PieceType::Pawn),
					'P' => row.pieces[i] = Piece::black_piece(PieceType::Pawn),
					'r' => row.pieces[i] = Piece::white_piece(PieceType::Rook),
					'R' => row.pieces[i] = Piece::black_piece(PieceType::Rook),
					'n' => row.pieces[i] = Piece::white_piece(PieceType::Knight),
					'N' => row.pieces[i] = Piece::black_piece(PieceType::Knight),
					'b' => row.pieces[i] = Piece::white_piece(PieceType::Bishop),
					'B' => row.pieces[i] = Piece::black_piece(PieceType::Bishop),
					'q' => row.pieces[i] = Piece::white_piece(PieceType::Queen),
					'Q' => row.pieces[i] = Piece::black_piece(PieceType::Queen),
					'k' => row.pieces[i] = Piece::white_piece(PieceType::King),
					'K' => row.pieces[i] = Piece::black_piece(PieceType::King),
					'1' => row.pieces[i] = Piece::air(),
					'2' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
					}
					'3' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
					}
					'4' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
						row.pieces[i + 3] = Piece::air();
					}
					'5' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
						row.pieces[i + 3] = Piece::air();
						row.pieces[i + 4] = Piece::air();
					}
					'6' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
						row.pieces[i + 3] = Piece::air();
						row.pieces[i + 4] = Piece::air();
						row.pieces[i + 5] = Piece::air();
					}
					'7' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
						row.pieces[i + 3] = Piece::air();
						row.pieces[i + 4] = Piece::air();
						row.pieces[i + 5] = Piece::air();
						row.pieces[i + 6] = Piece::air();
					}
					'8' => {
						row.pieces[i] = Piece::air();
						row.pieces[i + 1] = Piece::air();
						row.pieces[i + 2] = Piece::air();
						row.pieces[i + 3] = Piece::air();
						row.pieces[i + 4] = Piece::air();
						row.pieces[i + 5] = Piece::air();
						row.pieces[i + 6] = Piece::air();
						row.pieces[i + 7] = Piece::air();
					}
					_ => unreachable!("Unknown values in input FEN notation!")
				}
			}
			
			rows.push(row);
		}
		
		Fen {
			rows
		}
	}
}

impl Row {
	pub fn empty() -> Self {
		Row {
			pieces: vec![Piece::air(); 8]
		}
	}
}

impl Piece {
	pub fn air() -> Self {
		Piece {
			piece_type: PieceType::Empty,
			color: PieceColor::Empty
		}
	}
	
	pub fn white_piece(piece_type: PieceType) -> Self {
		Piece {
			piece_type,
			color: PieceColor::White
		}
	}

	pub fn black_piece(piece_type: PieceType) -> Self {
		Piece {
			piece_type,
			color: PieceColor::Black
		}
	}
}

impl Default for Fen {
	/// The starting position for a chess game
	fn default() -> Self {
		let pawn_row_white = vec![Piece::white_piece(PieceType::Pawn); 8];
		let pawn_row_black = vec![Piece::black_piece(PieceType::Pawn); 8];
		let king_row_white =
			vec![
				Piece::white_piece(PieceType::Rook),
				Piece::white_piece(PieceType::Knight),
				Piece::white_piece(PieceType::Bishop),
				Piece::white_piece(PieceType::Queen),
				Piece::white_piece(PieceType::King),
				Piece::white_piece(PieceType::Bishop),
				Piece::white_piece(PieceType::Knight),
				Piece::white_piece(PieceType::Rook),
			];
		let king_row_black =
			vec![
				Piece::black_piece(PieceType::Rook),
				Piece::black_piece(PieceType::Knight),
				Piece::black_piece(PieceType::Bishop),
				Piece::black_piece(PieceType::Queen),
				Piece::black_piece(PieceType::King),
				Piece::black_piece(PieceType::Bishop),
				Piece::black_piece(PieceType::Knight),
				Piece::black_piece(PieceType::Rook),
			];
		
		let row_1 = Row {
			pieces: king_row_white
		};
		let row_2 = Row {
			pieces: pawn_row_white
		};
		let row_3 = Row::empty();
		let row_4 = Row::empty();
		let row_5 = Row::empty();
		let row_6 = Row::empty();
		let row_7 = Row {
			pieces: pawn_row_black
		};
		let row_8 = Row {
			pieces: king_row_black
		};
		
		let rows = vec![row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8];
		
		Fen {
			rows
		}
	}
}

impl Default for Row {
	fn default() -> Self {
		Row::empty()
	}
}

impl Default for Piece {
	fn default() -> Self {
		Piece::air()
	}
}

impl Display for Fen {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for (i, row) in self.rows.iter().enumerate() {
			if i == 8 {
				write!(f, "{row}")?;
			} else {
				write!(f, "{row}")?;
				write!(f, "/")?;
			}
		}
		write!(f, "")
	}
}

impl Display for Row {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let pieces = self.pieces.iter();
		let mut count: u16 = 0;
		let mut last_was_empty = false;
		let mut output_string= String::new();

		for piece in pieces {
			if piece.piece_type == PieceType::Empty {
				last_was_empty = true;
				count += 1;
			} else {
				if last_was_empty {
					output_string.push_str(&count.to_string());
					count = 0;
				}
				last_was_empty = false;
				output_string.push_str(&piece.to_string());
			}
		}

		write!(f, "{output_string}")
	}
}

impl Display for Piece {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.piece_type {
			PieceType::Pawn => {
				if self.color == PieceColor::White {
					write!(f, "p")
				} else {
					write!(f, "P")
				}
			}
			PieceType::Rook => {
				if self.color == PieceColor::White {
					write!(f, "r")
				} else {
					write!(f, "R")
				}
			}
			PieceType::Knight => {
				if self.color == PieceColor::White {
					write!(f, "n")
				} else {
					write!(f, "N")
				}
			}
			PieceType::Bishop => {
				if self.color == PieceColor::White {
					write!(f, "b")
				} else {
					write!(f, "B")
				}
			}
			PieceType::Queen => {
				if self.color == PieceColor::White {
					write!(f, "q")
				} else {
					write!(f, "Q")
				}
			}
			PieceType::King => {
				if self.color == PieceColor::White {
					write!(f, "k")
				} else {
					write!(f, "K")
				}
			}
			PieceType::Empty => {
				write!(f, "_")
			}
		}
	}
}