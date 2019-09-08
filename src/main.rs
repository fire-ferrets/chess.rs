mod board;

fn main() {
    let mut board = board::Board{fields: [[0; 8]; 8]};
    board.init_board();
    println!("{}", board.generate_board());
    return;
    /*
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     * 0,0,0,0
     */
}
