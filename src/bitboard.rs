use crate::{BIT_DECK, BOARD_SIZE, BONUS_DIV, BONUS_ODDS, INITIAL_STEPS, INIT_HIGHEST};

extern crate ndarray;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};
use std::collections::{HashMap, VecDeque};

const BIT_BONUS_DIV: u64 = BONUS_DIV as u64;
const MASK: u64 = 0b1111 as u64;
const GROUP_MASK: [u64; 2] = [
    0b1111111111111111,
    0b1111000000000000111100000000000011110000000000001111,
];
const STRIDES: [usize; 2] = [4, 1];

pub struct ThreesGameBitBoard {
    board: u64,
    deck: VecDeque<u64>,
    highest: u64,
    rng: ThreadRng,
    next: u32,
    lookup_table: [HashMap<u64, u64>; 2],
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
    if stride == STRIDES[0] {
        return row.swap_bytes() >> 8;
    } else {
        let mut row: u64 = row;
        let mut new: u64 = 0;
        for i in 0..BOARD_SIZE {
            new <<= 4;
            new |= row & MASK;
            row >>= 4;
        }
        return new;
    }
}

fn stride_row(row: u64, stride: usize) -> u64 {
    let mut ret: u64 = 0;
    let stride = stride - 1;
    for i in 0..BOARD_SIZE {
        ret |= (row & (MASK << 4 * i)) << (stride * i);
    }
    ret
}

fn slide_row(row: u64, stride: usize) -> u64 {
    let mut mask: u64 = MASK;
    let mut new: u64 = 0;
    let mut prev: u64 = 16;
    let mut inc: u64 = 1;
    for _ in 0..BOARD_SIZE {
        if prev == 0 {
            new |= (mask & row) >> 4 * stride;
        } else if mask & row > inc << 1 {
            if prev == mask & row {
                new |= prev + inc;
            } else {
                new |= mask & row;
            }
        } else {
            if prev + (mask & row) == inc + (inc << 1) {
                new |= inc + (inc << 1);
            } else {
                new |= mask & row;
            }
        }
        prev = mask & new;
        mask <<= 4 * stride;
        prev <<= 4 * stride;
        inc <<= 4 * stride;
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
    let mut fwd_lookup_table: HashMap<u64, u64> = HashMap::new();
    let mut bwd_lookup_table: HashMap<u64, u64> = HashMap::new();

    // init board
    // first 4 bytes will be top left, first dim is rows second is cols
    let open_squares: [usize; BOARD_SIZE * BOARD_SIZE - 1] = core::array::from_fn(|i| i + 1);
    for (_, sq) in open_squares
        .choose_multiple(&mut rng, INITIAL_STEPS)
        .enumerate()
    {
        board |= deck.pop_front().unwrap_or(0) << (4 * sq);
        if deck.len() == 0 {
            deck = new_deck(&mut rng);
        }
    }

    // init hashmap LUT
    for stride in STRIDES {
        for row in 0..((1 as u64) << (4 * BOARD_SIZE)) {
            let strided: u64 = stride_row(row, stride);
            let slided: u64 = slide_row(row, stride);
            for shift in 0..4 {
                fwd_lookup_table.insert(
                    strided << (shift * 4 * (5 - stride)),
                    slided << (shift * 4 * (5 - stride)),
                );
                bwd_lookup_table.insert(
                    reverse_row(strided, stride) << (shift * 4 * (5 - stride)),
                    reverse_row(slided, stride) << (shift * 4 * (5 - stride)),
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
        lookup_table: [fwd_lookup_table, bwd_lookup_table],
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
                board >>= 4;
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
            self.highest / BIT_BONUS_DIV,
            self.highest / BIT_BONUS_DIV / 2,
            self.highest / BIT_BONUS_DIV / 4,
        ];
    }

    fn draw_card(&mut self) -> u64 {
        let card: u64;

        if self.next == 0 {
            let bonuses: [u64; 3] = [
                self.highest / BIT_BONUS_DIV,
                self.highest / BIT_BONUS_DIV / 2,
                self.highest / BIT_BONUS_DIV / 4,
            ];
            card = *bonuses
                .choose(&mut self.rng)
                .unwrap_or(&(self.highest / BIT_BONUS_DIV));
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
            match self.lookup_table[(dir & 2) >> 1]
                .get(&(self.board & (GROUP_MASK[dir & 1] << (STRIDES[dir & 1] * i * 4))))
            {
                Some(row) => {
                    new |= row << (STRIDES[dir & 1] * i * 4);
                    println!("FOUND ROW")
                }
                None => println!("NO ROW FOUND"),
            }
        }
        // TODO: ADD CARD, BIT BONUS DIV
        self.board = new;
    }

    pub fn calculate_score(&self) -> u32 {
        let mut board = self.board;
        let mut score: u32 = 0;
        for _ in 0..BOARD_SIZE {
            for __ in 0..BOARD_SIZE {
                if board & MASK > 2 {
                    score += 3_u32.pow((board & MASK) as u32 - 1);
                }
                board >>= 4;
            }
        }
        return score;
    }
}
