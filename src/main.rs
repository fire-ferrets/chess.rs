// board.rs
trait Generate {
    fn generate_board(&self) -> String;
}

struct Board {
    // holds an integer for every field, defining what piece is
    // on the field at the moment
    fields: Vec<Vec<i32>>
}

impl Generate for Board {
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
}

fn main() {
    let mut inner: Vec<i32> = vec![0; 4];
    let mut vec: Vec<Vec<i32>> = vec![inner; 4];
    let board = Board{fields: vec};
    println!("{}", board.generate_board());
    /*
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     */
}
