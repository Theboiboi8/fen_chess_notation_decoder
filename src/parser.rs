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
	#[allow(clippy::too_many_lines)]
	pub fn from_string(input: &str) -> Self {
		println!("Parsing input: {input}");
		
		let mut rows = Vec::<Row>::with_capacity(8);
		
		for input_row in input.split('/') {
			let mut row = Row::empty();
			let mut last_index: Option<usize> = None;
			
			for (i, char) in input_row.chars().enumerate() {
				assert!(i <= 7, "More than 8 characters provided in input row {input_row}");
				
				let index: usize = if last_index.is_none() {
					i
				} else {
					#[allow(clippy::unnecessary_unwrap)]
					last_index.unwrap()
				};
				
				match char {
					'p' => {
						row.pieces[index] = Piece::white_piece(PieceType::Pawn);
						last_index = Some(index + 1);
					},
					'P' => {
						row.pieces[index] = Piece::black_piece(PieceType::Pawn);
						last_index = Some(index + 1);
					},
					'r' => {
						row.pieces[index] = Piece::white_piece(PieceType::Rook);
						last_index = Some(index + 1);
					},
					'R' => {
						row.pieces[index] = Piece::black_piece(PieceType::Rook);
						last_index = Some(index + 1);
					},
					'n' => {
						row.pieces[index] = Piece::white_piece(PieceType::Knight);
						last_index = Some(index + 1);
					},
					'N' => {
						row.pieces[index] = Piece::black_piece(PieceType::Knight);
						last_index = Some(index + 1);
					},
					'b' => {
						row.pieces[index] = Piece::white_piece(PieceType::Bishop);
						last_index = Some(index + 1);
					},
					'B' => {
						row.pieces[index] = Piece::black_piece(PieceType::Bishop);
						last_index = Some(index + 1);
					},
					'q' => {
						row.pieces[index] = Piece::white_piece(PieceType::Queen);
						last_index = Some(index + 1);
					},
					'Q' => {
						row.pieces[index] = Piece::black_piece(PieceType::Queen);
						last_index = Some(index + 1);
					},
					'k' => {
						row.pieces[index] = Piece::white_piece(PieceType::King);
						last_index = Some(index + 1);
					},
					'K' => {
						row.pieces[index] = Piece::black_piece(PieceType::King);
						last_index = Some(index + 1);
					},
					'1' => {
						row.pieces[index] = Piece::air();
						last_index = Some(index + 1);
					},
					'2' => {
						for offset in 0..2 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 2);
					}
					'3' => {
						for offset in 0..3 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 3);
					}
					'4' => {
						for offset in 0..4 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 4);
					}
					'5' => {
						for offset in 0..5 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 5);
					}
					'6' => {
						for offset in 0..6 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 6);
					}
					'7' => {
						for offset in 0..7 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 7);
					}
					'8' => {
						for offset in 0..8 {
							row.pieces[index + offset] = Piece::air();
						}
						last_index = Some(index + 8);
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
		let mut output_string= String::new();
		
		for (i, row) in self.rows.iter().enumerate() {
			output_string.push_str(&row.to_string());
			if i < 7 {
				output_string.push('/');
			}
		}
		
		write!(f, "{output_string}")
	}
}

impl Display for Row {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let pieces = self.pieces.iter();
		let mut output_string= String::new();

		for piece in pieces {
			output_string.push_str(&piece.to_string());
		}
		
		output_string = output_string
			.replace("________", "8")
			.replace("_______", "7")
			.replace("______", "6")
			.replace("_____", "5")
			.replace("____", "4")
			.replace("___", "3")
			.replace("__", "2")
			.replace('_', "1");

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