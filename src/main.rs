struct Board {
    // holds an integer for every field, defining what piece is
    // on the field at the moment
    fields: Vec<Vec<i32>>
}

impl Board {
    fn generate_board(&self) -> String {
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

    /*
     * empty: 0
     * pawn:  1
     * rook:  2
     * knight:3
     * bishop:4
     * queen :5
     * king  :6
     */
    fn init_board(&mut self) {
        let first_row: Vec<i32> = vec![2, 3, 4, 5, 6, 4, 3, 2];
        let pawn_row: Vec<i32> = vec![1; 8];
        let empty_row: Vec<i32> = vec![0; 8];
        let first_white_row: Vec<i32> = vec![2, 3, 4, 6, 5, 4, 3, 2];
        self.fields = vec![first_row.clone(),
                           pawn_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           empty_row.clone(),
                           pawn_row.clone(),
                           first_white_row.clone()];
        return;
    }
}

fn main() {
    let mut board = Board{fields: vec![]};
    board.init_board();
    println!("{}", board.generate_board());
    /*
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     */
}
