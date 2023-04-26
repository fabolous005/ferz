#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum File {
    A, B, C, D, E, F, G, H
}

#[derive(Copy, Clone)]
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

fn main() {
    println!("Hello, world!");
}
