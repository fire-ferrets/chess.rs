pub trait Piece {
    pub fn legit_move(&self, move : tuple) -> bool;
    pub fn possible_moves() -> Vec<tuple>;
}

pub struct Rook {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Rook {
    pub fn legit_move(&self, move : tuple) -> bool {
        let mut legit = true;

        let x_src : i32 = move.0.0;
        let y_src : i32 = move.0.1;

        let x_end : i32 = move.1.0;
        let y_end : i32 = move.1.1;

        // Check horizontally
        for i in x_src..x_end {
            if (*(self.board_ptr))[y_src][i].color != 'e' {
                legit = false;
            }
        }

        // Check veritcally
        for i in y_src..y_end {
            if (*(self.board_ptr))[i][x_src].color != 'e' {
                legit = false;
            }
        }
        legit
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Bishop {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Bishop {
    pub fn legit_move(&self, move : tuple) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Knight {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Knight {
    pub fn legit_move(&self, move : tuple) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Queen {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Queen {
    pub fn legit_move(&self, move : tuple) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct King {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for King {
    pub fn legit_move(&self, move : tuple) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}

pub struct Pawn {
    pub color: char;
    pub board_ptr : &Board;
}

impl Piece for Pawn {
    pub fn legit_move(&self, move : tuple) -> bool {
    }

    fn possible_moves() -> Vec<tuple> {
    }
}
