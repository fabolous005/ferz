enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Clone, Copy)]
struct Square {
    file: File,
    rank: Rank,
}

enum File {
    A, B, C, D, E, F, G, H
}

enum Rank {
    R1, R2, R3, R4, R5, R6, R7, R8
}

struct CastlingRights {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}


struct Input {
    board: [Option<Piece>; 64],     // 8x8 chess board with optional pieces
    white_to_move: bool,            // color to move (either White or Black)
    en_passant: Option<Square>,     // en passant square, if any
    castling_rights: CastlingRights // castling rights for both sides
}

impl InputLayer {
    fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
