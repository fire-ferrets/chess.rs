pub trait Piece {
    pub fn legit_move(move) -> bool;
    pub fn possible_moves() -> Vec<tuple>;
}

pub struct Rook {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Rook {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Bishop {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Bishop {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Knight {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Knight {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Queen {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Queen {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct King {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for King {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Pawn {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Pawn {
    pub fn legit_move(move) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}
