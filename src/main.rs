mod bitboard;
mod game;

pub const DECK: [u32; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
pub const BIT_DECK: [u64; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
pub const BOARD_SIZE: usize = 4;
pub const INITIAL_STEPS: usize = 9;

pub const BONUS_ODDS: u32 = 21;
pub const BONUS_DIV: u32 = 8;

const INIT_HIGHEST: u32 = 96;
fn main() {
    // let mut threes: game::ThreesGame = game::init_threes_game();
    // println!("{}", threes.board_string());
    // println!("{}", threes.deck_string());
    // threes.swipe(0);
    // println!("{}", threes.board_string());
    // println!("{}", threes.deck_string());
    // threes.swipe(1);
    // println!("{}", threes.board_string());
    // println!("{}", threes.deck_string());
    // threes.swipe(2);
    // println!("{}", threes.board_string());
    // println!("{}", threes.deck_string());
    // threes.swipe(3);
    // println!("{}", threes.board_string());
    // println!("{}", threes.deck_string());
    // println!("{}", threes.calculate_score());

    let mut threes: bitboard::ThreesGameBitBoard = bitboard::init_threes_game_bitboard();
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(3);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(2);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(1);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    threes.swipe(0);
    println!("{}", threes.board_string());
    println!("{}", threes.deck_string());
    println!("{}", threes.calculate_score());
}
