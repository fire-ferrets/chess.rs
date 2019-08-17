mod board;

fn main() {
    let mut board = board::Board{fields: vec![]};
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
