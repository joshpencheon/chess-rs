mod piece;
mod position;

use position::Position;

fn main() {
    let position = Position::default();
    println!("{:?}", position);
}
