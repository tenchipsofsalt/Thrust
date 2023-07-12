use crate::{BOARD_SIZE, BONUS_DIV, BONUS_ODDS, DECK, INITIAL_STEPS, INIT_HIGHEST};

extern crate ndarray;

use ndarray::Array2;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use std::collections::VecDeque;

pub struct ThreesGame {
    board: Array2<u32>,
    deck: VecDeque<u32>,
    highest: u32,
    rng: ThreadRng,
    next: u32,
}

fn new_deck(rng: &mut ThreadRng) -> VecDeque<u32> {
    let mut nums = DECK;
    nums.shuffle(rng);
    let deck: VecDeque<u32> = nums.into();
    return deck;
}

pub fn init_threes_game() -> ThreesGame {
    // setup vars
    let mut board: Array2<u32> = Array2::<u32>::zeros((BOARD_SIZE, BOARD_SIZE));
    let mut rng = thread_rng();
    let mut deck: VecDeque<u32> = new_deck(&mut rng);
    let highest: u32 = INIT_HIGHEST;
    let next: u32 = rng.gen_range(0..BONUS_ODDS);

    // init board
    // [0][0] will be top left, first dim is rows second is cols
    board[[0, 0]] = highest;
    let open_squares: [usize; BOARD_SIZE * BOARD_SIZE - 1] = core::array::from_fn(|i| i + 1);
    for (_, sq) in open_squares
        .choose_multiple(&mut rng, INITIAL_STEPS)
        .enumerate()
    {
        board[[sq % BOARD_SIZE, sq / BOARD_SIZE]] = deck.pop_front().unwrap_or(3);
        if deck.len() == 0 {
            deck = new_deck(&mut rng);
        }
    }

    return ThreesGame {
        board: board,
        deck: deck,
        highest: highest,
        rng: rng,
        next: next,
    };
}

impl ThreesGame {
    pub fn deck_string(&self) -> String {
        return format!("{:?}", self.deck);
    }

    pub fn board_string(&self) -> String {
        return format!("{:?}", self.board);
    }

    pub fn next_is_bonus(&self) -> bool {
        return self.next == 0;
    }

    pub fn current_bonus_values(&self) -> [u32; 3] {
        return [
            self.highest / BONUS_DIV,
            self.highest / BONUS_DIV / 2,
            self.highest / BONUS_DIV / 4,
        ];
    }

    fn draw_card(&mut self) -> u32 {
        let card: u32;

        if self.next == 0 {
            let bonuses: [u32; 3] = [
                self.highest / BONUS_DIV,
                self.highest / BONUS_DIV / 2,
                self.highest / BONUS_DIV / 4,
            ];
            card = *bonuses
                .choose(&mut self.rng)
                .unwrap_or(&(self.highest / BONUS_DIV));
        } else {
            if self.deck.len() == 0 {
                self.deck = new_deck(&mut self.rng);
            }
            card = self.deck.pop_front().unwrap_or(3);
        }
        self.next = self.rng.gen_range(0..BONUS_ODDS);
        card
    }

    pub fn swipe(&mut self, dir: u32) {
        // 0 is right, then clockwise 1/2/3
        let mut changed: bool = false;
        let mut open: [usize; 4] = [0; 4];
        let mut idx: usize = 0;
        if dir == 0 {
            for r in 0..BOARD_SIZE {
                for c in 1..BOARD_SIZE {
                    let s: u32 = self.board[[r, c - 1]] + self.board[[r, c]];
                    if (s > 5 && s == self.board[[r, c]] << 1) || s == 3 || s == self.board[[r, c]]
                    {
                        self.board[[r, c - 1]] = s;
                        self.board[[r, c]] = 0;
                        if s > self.highest {
                            self.highest = s;
                        }
                        changed = true;
                    }
                }
                if self.board[[r, BOARD_SIZE - 1]] == 0 {
                    open[idx] = r;
                    idx += 1;
                }
            }
            if changed {
                if idx > 0 {
                    self.board[[
                        *open[0..idx].choose(&mut self.rng).unwrap_or(&0),
                        BOARD_SIZE - 1,
                    ]] = self.draw_card();
                }
            }
        } else if dir == 1 {
            for c in 0..BOARD_SIZE {
                for r in 1..BOARD_SIZE {
                    let s: u32 = self.board[[r - 1, c]] + self.board[[r, c]];
                    if (s > 5 && s == self.board[[r, c]] << 1) || s == 3 || s == self.board[[r, c]]
                    {
                        self.board[[r - 1, c]] = s;
                        self.board[[r, c]] = 0;
                        if s > self.highest {
                            self.highest = s;
                        }
                        changed = true;
                    }
                }
                if self.board[[BOARD_SIZE - 1, c]] == 0 {
                    open[idx] = c;
                    idx += 1;
                }
            }
            if changed {
                if idx > 0 {
                    self.board[[
                        BOARD_SIZE - 1,
                        *open[0..idx].choose(&mut self.rng).unwrap_or(&0),
                    ]] = self.draw_card();
                }
            }
        } else if dir == 2 {
            for r in 0..BOARD_SIZE {
                for c in (0..BOARD_SIZE - 1).rev() {
                    let s: u32 = self.board[[r, c + 1]] + self.board[[r, c]];
                    if (s > 5 && s == self.board[[r, c]] << 1) || s == 3 || s == self.board[[r, c]]
                    {
                        self.board[[r, c + 1]] = s;
                        self.board[[r, c]] = 0;
                        if s > self.highest {
                            self.highest = s;
                        }
                        changed = true;
                    }
                }
                if self.board[[r, 0]] == 0 {
                    open[idx] = r;
                    idx += 1;
                }
            }
            if changed {
                if idx > 0 {
                    self.board[[*open[0..idx].choose(&mut self.rng).unwrap_or(&0), 0]] =
                        self.draw_card();
                }
            }
        } else {
            for c in 0..BOARD_SIZE {
                for r in (0..BOARD_SIZE - 1).rev() {
                    let s: u32 = self.board[[r + 1, c]] + self.board[[r, c]];
                    if (s > 5 && s == self.board[[r, c]] << 1) || s == 3 || s == self.board[[r, c]]
                    {
                        self.board[[r + 1, c]] = s;
                        self.board[[r, c]] = 0;
                        if s > self.highest {
                            self.highest = s;
                        }
                        changed = true;
                    }
                }
                if self.board[[0, c]] == 0 {
                    open[idx] = c;
                    idx += 1;
                }
            }
            if changed {
                if idx > 0 {
                    self.board[[0, *open[0..idx].choose(&mut self.rng).unwrap_or(&0)]] =
                        self.draw_card();
                }
            }
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score: u32 = 0;
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if self.board[[r, c]] < 3 {
                    continue;
                }
                score += 3_u32.pow((self.board[[r, c]] / 3).trailing_zeros() + 1);
            }
        }
        return score;
    }
}
