mod bitboard;

extern crate pancurses;

use bitboard::ThreesGameBitBoard;
use pancurses::{endwin, initscr, noecho, Input};

pub const DECK: [u32; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
pub const BIT_DECK: [u64; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
pub const BOARD_SIZE: usize = 4;
pub const INITIAL_STEPS: usize = 9;

pub const BONUS_ODDS: u32 = 21;
pub const BONUS_DIV: u32 = 8;

const INIT_HIGHEST: u32 = 96;

fn print_game_status(game: &ThreesGameBitBoard, window: &pancurses::Window) {
    window.clear();
    window.printw("Welcome to Threes! Press any char to quit, and arrow keys to play!\n");
    window.printw(format!("{}", game.board_string()));
    if game.next_is_bonus() {
        window.printw(format!("Next: {:?}\n", game.current_bonus_values()));
    } else {
        window.printw(format!(
            "Next: {}\n",
            game.deck_string().chars().next().unwrap()
        ));
    }
    window.printw(format!("Score: {}", game.calculate_score()));
    window.refresh();
}
fn main() {
    let window = initscr();
    let mut threes: bitboard::ThreesGameBitBoard = bitboard::init_threes_game_bitboard();
    window.keypad(true);
    print_game_status(&threes, &window);
    noecho();
    loop {
        match window.getch() {
            Some(Input::KeyLeft) => {
                threes.swipe(0);
            }
            Some(Input::KeyUp) => {
                threes.swipe(1);
            }
            Some(Input::KeyRight) => {
                threes.swipe(2);
            }
            Some(Input::KeyDown) => {
                threes.swipe(3);
            }
            Some(_) => {
                break;
            }
            None => (),
        }
        print_game_status(&threes, &window);
    }
    endwin();
}
