pub struct Board {
    // holds an integer for every field, defining what piece is
    // on the field at the moment
    pub fields: [[i32; 8]; 8]
}

impl Board {
    pub fn generate_board(&self) -> String {
        let mut result: String = "".to_owned();
        for row in 0..self.fields.len(){
            for col in 0..self.fields[0].len() {
                result = result + &self.fields[row][col].to_string();
                if col != self.fields[0].len() - 1 {
                    result = result + ",";
                }
            }
            if row != self.fields.len() - 1 {
                result = result + "\n";
            }
        }
        return result;
    }

    /**
     * empty: 0
     * pawn:  1
     * rook:  2
     * knight:3
     * bishop:4
     * queen :5
     * king  :6
     **/
    pub fn init_board(&mut self) {
        let first_row: [i32; 8] = [2, 3, 4, 5, 6, 4, 3, 2];
        let pawn_row: [i32; 8] = [1; 8];
        let empty_row: [i32; 8] = [0; 8];
        self.fields = [first_row.clone(),
                           pawn_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           pawn_row.clone(),
                           first_row.clone()];
        return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board(){
        Board{fields: [[0; 8]; 8]};
    }

    #[test]
    fn test_init_board(){
        let mut board: Board = Board{fields: [[0; 8]; 8]};
        board.init_board();
        let new_fields: [[i32; 8]; 8] =[
            [2,3,4,5,6,4,3,2],
            [1,1,1,1,1,1,1,1],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [1,1,1,1,1,1,1,1],
            [2,3,4,5,6,4,3,2]
        ];
        assert_eq!(board.fields, new_fields);
    }

    #[test]
    fn test_generate_board(){
        let mut board: Board = Board{fields: [[0; 8]; 8]};
        board.init_board();
        let new_string = String::from("2,3,4,5,6,4,3,2\n1,1,1,1,1,1,1,1\n0,0,0,0,0,0,0,0\n0,0,0,0,0,0,0,0\n0,0,0,0,0,0,0,0\n0,0,0,0,0,0,0,0\n1,1,1,1,1,1,1,1\n2,3,4,5,6,4,3,2");
        assert_eq!(board.generate_board(), new_string);
    }
}