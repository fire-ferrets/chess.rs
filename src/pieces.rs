mod board;

pub trait Piece {
    pub fn valid_move(&self, move : tuple) -> i32;
    pub fn check_way(&self, move : tuple) -> i32;
    pub fn possible_moves() -> Vec<tuple>;
}

pub struct Rook {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Rook {
    pub fn valid_move(&self, move : tuple) -> bool {
        let mut valid = false;

        let same_x = move.0.0 == move.1.0;
        let same_y = move.0.1 == move.1.1;

        if same_x ^ same_y {
            valid = true;
        }
        valid
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Bishop {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Bishop {
    pub fn valid_move(&self, move : tuple) -> bool {
        let mut valid = false;

        let diff_x = move.0.0 - move.1.0;
        let diff_y = move.0.1 - move.1.1;

        if diff_x.abs() == diff_y.abs() {
            valid = true;
        }
        valid
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Knight {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Knight {
    pub fn valid_move(&self, move : tuple) -> bool {
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Queen {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Queen {
    pub fn valid_move(&self, move : tuple) -> bool {
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct King {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for King {
    pub fn valid_move(&self, move : tuple) -> bool {
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Pawn {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Pawn {
    pub fn valid_move(&self, move : tuple) -> bool {
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct EmptyPiece {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for EmptyPiece {
    pub fn valid_move(&self, move : tuple) -> bool {
        false
    }

    pub fn check_way(&self, move : tuple) -> i32 {
    }

    fn possible_moves() -> Vec<tuple> {
        let empty_vec = Vec::new(); // Does this work?????
        empty_vec
    }
}
