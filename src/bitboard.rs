use crate::{BIT_DECK, BOARD_SIZE, BONUS_DIV, BONUS_ODDS, INITIAL_STEPS, INIT_HIGHEST};

extern crate ndarray;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use std::collections::{HashMap, VecDeque};

const BIT_BONUS_SUB: u64 = BONUS_DIV.trailing_zeros() as u64;
const SQ_BITS: usize = 4;
const MASK: u64 = 2_u64.pow(SQ_BITS as u32) - 1;
const GROUP_MASK: [u64; 2] = [
    0b1111111111111111,
    0b1111000000000000111100000000000011110000000000001111,
];
// 0 -> 3, 7, 11, 15
// 1 -> 12 - 15
// 2 -> 0, 4, 8, 12
// 3 -> 0, 1, 2, 3
const POSSIBLE_OPENINGS: [[usize; 4]; 4] = [
    [3, 7, 11, 15],
    [12, 13, 14, 15],
    [0, 4, 8, 12],
    [0, 1, 2, 3],
];
const STRIDES: [usize; 2] = [1, BOARD_SIZE];

pub struct ThreesGameBitBoard {
    board: u64,
    deck: VecDeque<u64>,
    highest: u64,
    rng: ThreadRng,
    next: u32,
    lookup_table: [HashMap<u64, u64>; 4],
}

fn num_to_bit(num: u32) -> u64 {
    let num: u64 = num as u64;
    if num < 3 {
        return num;
    } else {
        return (num / 3).trailing_zeros() as u64 + 3;
    }
}

fn bit_to_num(bit: u64) -> u32 {
    let bit = bit as u32;
    if bit < 3 {
        return bit;
    } else {
        return 2_u32.pow(bit - 3) * 3;
    }
}

fn new_deck(rng: &mut ThreadRng) -> VecDeque<u64> {
    let mut nums: [u64; 12] = BIT_DECK;
    nums.shuffle(rng);
    let deck: VecDeque<u64> = nums.into();
    return deck;
}

fn reverse_row(row: u64, stride: usize) -> u64 {
    if stride == STRIDES[1] {
        return row.swap_bytes() >> 8;
    } else {
        let mut row: u64 = row;
        let mut new: u64 = 0;
        for _ in 0..BOARD_SIZE {
            new <<= SQ_BITS;
            new += row & MASK;
            row >>= SQ_BITS;
        }
        return new;
    }
}

fn stride_row(row: u64, stride: usize) -> u64 {
    let mut ret: u64 = 0;
    let stride: usize = stride - 1;
    for i in 0..BOARD_SIZE {
        ret += (row & (MASK << SQ_BITS * i)) << (stride * i * SQ_BITS);
    }
    ret
}

fn slide_row(row: u64, stride: usize) -> u64 {
    let mut mask: u64 = MASK;
    let mut new: u64 = mask & row;
    let mut prev: u64 = new;
    let mut inc: u64 = 1;
    for _ in 1..BOARD_SIZE {
        mask <<= SQ_BITS * stride;
        inc <<= SQ_BITS * stride;
        prev <<= SQ_BITS * stride;
        if prev == 0 {
            new += (mask & row) >> SQ_BITS * stride;
        } else if mask & row > inc << 1 {
            if prev == (mask & row) {
                new += inc >> SQ_BITS * stride;
            } else {
                new += mask & row;
            }
        } else {
            if prev + (mask & row) == inc + (inc << 1) {
                new += (mask & row) >> SQ_BITS * stride;
            } else {
                new += mask & row;
            }
        }
        prev = mask & new;
    }
    new
}

pub fn init_threes_game_bitboard() -> ThreesGameBitBoard {
    // setup vars
    let highest: u64 = num_to_bit(INIT_HIGHEST);
    let mut board: u64 = highest;
    let mut rng = thread_rng();
    let mut deck: VecDeque<u64> = new_deck(&mut rng);
    let next: u32 = rng.gen_range(0..BONUS_ODDS);
    let mut dir_lut: [HashMap<u64, u64>; 4] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    // init board
    // first 4 bytes will be top left, first dim is rows second is cols
    let open_squares: [usize; BOARD_SIZE * BOARD_SIZE - 1] = core::array::from_fn(|i| i + 1);
    for (_, sq) in open_squares
        .choose_multiple(&mut rng, INITIAL_STEPS)
        .enumerate()
    {
        board += deck.pop_front().unwrap_or(0) << (SQ_BITS * sq);
        if deck.len() == 0 {
            deck = new_deck(&mut rng);
        }
    }

    // init hashmap LUT
    for row in 0..((1 as u64) << (SQ_BITS * BOARD_SIZE)) {
        for dir in 0..4 {
            let stride = STRIDES[dir & 1];
            let mut strided: u64 = stride_row(row, stride);
            let mut slided: u64 = slide_row(strided, stride);
            if dir > 1 {
                strided = reverse_row(strided, stride);
                slided = reverse_row(slided, stride);
            }
            for shift in 0..4 {
                dir_lut[dir].insert(
                    strided << (shift * SQ_BITS * (BOARD_SIZE + 1 - stride)),
                    slided << (shift * SQ_BITS * (BOARD_SIZE + 1 - stride)),
                );
            }
        }
    }

    return ThreesGameBitBoard {
        board: board,
        deck: deck,
        highest: highest,
        rng: rng,
        next: next,
        lookup_table: dir_lut,
    };
}

impl ThreesGameBitBoard {
    pub fn deck_string(&self) -> String {
        let mut str: String = String::new();
        for card in &self.deck {
            str.push_str(&format!("{} ", bit_to_num(*card)));
        }
        str.push('\n');
        return str;
    }

    pub fn board_string(&self) -> String {
        let mut board = self.board;
        let mut str: String = String::new();
        for _ in 0..BOARD_SIZE {
            for __ in 0..BOARD_SIZE {
                str.push_str(&format!("{} ", bit_to_num(MASK & board)));
                board >>= SQ_BITS;
            }
            str.push('\n');
        }
        return str;
    }

    pub fn next_is_bonus(&self) -> bool {
        return self.next == 0;
    }

    pub fn current_bonus_values(&self) -> [u64; 3] {
        return [
            self.highest - BIT_BONUS_SUB,
            self.highest - BIT_BONUS_SUB - 1,
            self.highest - BIT_BONUS_SUB - 2,
        ];
    }

    fn draw_card(&mut self) -> u64 {
        let card: u64;

        if self.next == 0 {
            let bonuses: [u64; 3] = self.current_bonus_values();
            card = *bonuses
                .choose(&mut self.rng)
                .unwrap_or(&(self.highest - BIT_BONUS_SUB));
        } else {
            if self.deck.len() == 0 {
                self.deck = new_deck(&mut self.rng);
            }
            card = self.deck.pop_front().unwrap_or(3);
        }
        self.next = self.rng.gen_range(0..BONUS_ODDS);
        card
    }

    pub fn swipe(&mut self, dir: usize) {
        // 0 is right, then clockwise 1/2/3
        let mut new: u64 = 0;
        for i in 0..BOARD_SIZE {
            match self.lookup_table[dir].get(
                &(self.board
                    & (GROUP_MASK[dir & 1] << ((BOARD_SIZE + 1 - STRIDES[dir & 1]) * i * SQ_BITS))),
            ) {
                Some(row) => {
                    new += row;
                }
                None => panic!("Row not found!"),
            }
        }

        // add card if changed
        if self.board != new {
            let mut open: [usize; 4] = [0; 4];
            let mut idx = 0;
            for end in POSSIBLE_OPENINGS[dir] {
                if new & (MASK << (end * SQ_BITS)) == 0 {
                    open[idx] = end;
                    idx += 1;
                }
            }
            new +=
                self.draw_card() << (*open[0..idx].choose(&mut self.rng).unwrap_or(&0) * SQ_BITS);
            self.board = new;
            // update highest
        }
    }

    pub fn calculate_score(&self) -> u32 {
        let mut board = self.board;
        let mut score: u32 = 0;
        for _ in 0..BOARD_SIZE {
            for __ in 0..BOARD_SIZE {
                if board & MASK > 3 {
                    score += 3_u32.pow((board & MASK) as u32 - 2);
                }
                board >>= SQ_BITS;
            }
        }
        return score;
    }
}
