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
