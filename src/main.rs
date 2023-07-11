mod game;
fn main() {
    let mut threes: game::ThreesGame = game::init_threes_game();
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(0);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(1);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(2);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(3);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    println!("{}", threes.calculate_score());
}
