#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    colour: Colour,
    file: File,
    rank: Rank
}

impl Piece {
    pub fn new() -> Self {
        Self {
            piece_type: PieceType::Knight,
            colour: Colour::White,
            file: 'a',
            rank: 1
        }
    }
}

#[derive(Debug)]
pub enum Colour {
    White,
    Black
}


// TODO: model as separate structs that implement
// a common trait for being a "piece".
//
#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

// TODO: model location as a tuple struct for private bounded
// co-ords, with a constructor that can parse rank/file.
type File = char;
type Rank = i8;
