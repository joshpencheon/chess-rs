use crate::piece::Piece;

#[derive(Debug)]
pub struct Position {
    pieces: Vec<Piece>
}

impl Default for Position {
    fn default() -> Self {
        // TODO: try using Box<dyn Piece> here
        // with Piece as a trait instead.
        let mut pieces = vec![];

        let mut add_pieces = || {
            for _ in 1..3 {
                pieces.push(Piece::new());
            }
        };

        add_pieces();

        Self { pieces }
    }
}
