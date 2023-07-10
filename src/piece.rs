use crate::board::Board;

#[derive(Copy, Clone, Debug)]
pub enum PieceKind {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
    Empty,
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    pub fn new_king(color: Color) -> Self {
        Self {
            kind: PieceKind::King,
            color,
        }
    }

    pub fn new_queen(color: Color) -> Self {
        Self {
            kind: PieceKind::Queen,
            color,
        }
    }

    pub fn new_bishop(color: Color) -> Self {
        Self {
            kind: PieceKind::Bishop,
            color,
        }
    }

    pub fn new_knight(color: Color) -> Self {
        Self {
            kind: PieceKind::Knight,
            color,
        }
    }

    pub fn new_rook(color: Color) -> Self {
        Self {
            kind: PieceKind::Rook,
            color,
        }
    }

    pub fn new_pawn(color: Color) -> Self {
        Self {
            kind: PieceKind::Pawn,
            color,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            kind: PieceKind::Empty,
            color: Color::Black,
        }
    }

    pub fn turn_allowed(&self, board: &Board, turn: [[usize; 2]; 2]) -> bool {
        todo!()
    }
}
