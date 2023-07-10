use crate::piece::Color;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Board {
    // holds an integer for every field, defining what piece is
    // on the field at the moment
    pub fields: [[Piece; 8]; 8],
}

impl Default for Board {
    fn default() -> Self {
        let mut fields = [[Piece::new_empty(); 8]; 8];
        fields[0][0] = Piece::new_rook(Color::Black);
        fields[0][7] = Piece::new_rook(Color::Black);
        fields[0][1] = Piece::new_knight(Color::Black);
        fields[0][6] = Piece::new_knight(Color::Black);
        fields[0][2] = Piece::new_bishop(Color::Black);
        fields[0][5] = Piece::new_bishop(Color::Black);
        fields[0][3] = Piece::new_king(Color::Black);
        fields[0][4] = Piece::new_queen(Color::Black);
        fields[1] = [Piece::new_pawn(Color::Black); 8];
        fields[7][0] = Piece::new_rook(Color::White);
        fields[7][7] = Piece::new_rook(Color::White);
        fields[7][1] = Piece::new_knight(Color::White);
        fields[7][6] = Piece::new_knight(Color::White);
        fields[7][2] = Piece::new_bishop(Color::White);
        fields[7][5] = Piece::new_bishop(Color::White);
        fields[7][3] = Piece::new_queen(Color::White);
        fields[7][4] = Piece::new_king(Color::White);
        fields[6] = [Piece::new_pawn(Color::White); 8];
        Board { fields }
    }
}

impl Board {
    pub fn turn_allowed(&self, turn: [[usize; 2]; 2]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        Board::default();
    }

}
