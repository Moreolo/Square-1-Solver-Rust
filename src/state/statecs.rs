use crate::square1::Square1;

use super::State;

pub struct StateCS {
    sq1: Square1,
    cubeshape: usize,
    parity: usize,
    index: usize
}

impl State for StateCS {
    const NAME: &str = "cs";

    const SIZE: usize = 113;

    const MAX_SLICES: u8 = 7;

    fn new(sq1: Square1) -> Self {
        let mut state: Self = Self {sq1, cubeshape: 0, parity: 0, index: 0};
        state.calc_index();
        state
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_symmetric_indecies(&mut self) -> Vec<usize> {
        vec![]
    }

    fn get_square1_num(&self) -> u64 {
        self.sq1.get_num()
    }

    fn gen_next_positions(sq1num: u64) -> Vec<u64> {
        let mut opened = vec![];
        for mirror in [false, true] {
            let mut base = Square1::from_num(sq1num);
            if mirror { base.mirror_layers(); }
            for turn in base.get_unique_turns() {
                let mut adj = base.clone();
                adj.turn_layers(&turn);
                adj.turn_slice().expect("Unique Turns is wrong");
                opened.push(adj.get_num());
            }
        }
        opened
    }
}

impl StateCS {
    fn calc_index(&mut self) {
        let mut up_shape = self.get_shape(true);
        let mut down_shape = self.get_shape(false);
        if up_shape.len() > 4 {
            self.sq1.flip_layers();
            (up_shape, down_shape) = (down_shape, up_shape);
        }
        match match 4 - up_shape.len() {
            0 => {
                let mut up_case = Self::get_case_4e(&up_shape);
                let mut down_case = Self::get_case_4e(&down_shape);

                if (up_case > 7 || down_case > 7) && (
                    ! (up_case == 1 || up_case == 5 || down_case == 1 || down_case == 5) ||
                    (up_case == 9 && down_case == 1 || up_case == 1 && down_case == 9)
                ) {
                    self.sq1.mirror_layers_atd(8);
                    up_case = Self::mirror_case_4e(up_case);
                    down_case = Self::mirror_case_4e(down_case);
                }

                if up_case < down_case {
                    self.sq1.flip_layers();
                    (up_shape, down_shape) = (down_shape, up_shape);
                    (up_case, down_case) = (down_case, up_case);
                }

                if up_case == 8 {
                    if down_case == 1 {
                        down_case = 0;
                    } else {
                        down_case = 1;
                    }
                } else if up_case == 9 {
                    up_case = 8;
                    down_case = 2;
                }

                self.cubeshape = (up_case * (up_case + 1) / 2) + down_case;
                self.sq1.turn_layers(&(self.get_shape_turn(true, up_shape), self.get_shape_turn(false, down_shape)));
                true
            },
            1 => {
                let mut up_case = Self::get_case_6e(&up_shape);
                let down_case = Self::get_case_2e(&down_shape);
                if up_case > 6 {
                    self.sq1.mirror_layers_atd(9);
                    up_case -= 7;
                }
                self.cubeshape = 39 + 3 * up_case + down_case;
                self.sq1.turn_layers(&(self.get_shape_turn(true, up_shape), self.get_shape_turn(false, down_shape)));
                !(up_case > 2)
            },
            2 => {
                self.cubeshape = 60 + if let Some(min) = up_shape.iter().min() {*min} else {0};
                false
            },
            _ => {false}
        } {
            true => {
                for i in 0..16 {
                    for j in 1..i {
                        if self.sq1.pieces[j] > self.sq1.pieces[i] {
                            self.parity += 1;
                        }
                    }
                }
                self.parity &= 1;
            },
            false => {}
        }
        self.index = self.parity * 65 + self.cubeshape;
    }

    fn get_shape(&self, for_up: bool) -> Vec<usize> {
        let mut shape: Vec<usize> = vec![0];
        let mut angle: u8 = 0;
        let mut turn: usize = 0;
        while angle < 12 {
            if self.sq1.pieces[if for_up {turn} else {15 - turn}] & 1 == 0 {
                shape.push(0);
                angle += 2;
            } else {
                if let Some(last) = shape.last_mut() {
                    *last += 1;
                }
                angle += 1;
            }
            turn += 1;
        }
        if shape[0] == 0 {
            shape.remove(0);
        } else {
            if let Some(elem) = shape.pop() {
                shape[0] += elem;
            }
        }
        shape
    }

    fn get_shape_turn(&self, for_up: bool, shape: Vec<usize>) -> usize {
        let highest: usize = if let Some(max) = shape.iter().max() {*max} else {0};
        let piece_count: usize = 12 - shape.len();

        let mut edge_count: usize = 0;
        let mut turn: usize = 0;
        while edge_count < highest {
            if self.sq1.pieces[if for_up {turn} else {15 - turn}] & 1 == 0 {
                edge_count = 0;
            } else {
                edge_count += 1;
            }
            turn = (turn + 1) % piece_count
        }

        if shape.iter().filter(|x| **x == highest).count() == 2 {
            if self.sq1.pieces[if for_up {turn + 1} else {15 - (turn + 1)}] & 1 != 0 {
                turn = (turn + 1 + highest) % piece_count;
            } else if highest == 1 {
                if self.sq1.pieces[if for_up {turn + 2} else {15 - (turn + 2)}] & 1 != 0 {
                    turn = (turn + 3) % piece_count;
                }
            }
        }
        
        if for_up {
            turn
        } else {
            (piece_count - turn) % piece_count
        }
    }

    fn get_case_4e(shape: &Vec<usize>) -> usize {
        let mut case: usize = if let Some(max) = shape.iter().max() {*max} else {0};
        case = 0.max(case as i8 - 2) as usize;
        let mut gap: usize = 1;
        for i in 0..4 {
            if shape[i] == 0 {
                let mut dist: usize = 0;
                while shape[(4 + i - 1 - dist) % 4] < 2 {
                    dist += 1;
                }
                case += gap + dist;
                if gap == 1 {
                    gap = 3;
                } else {
                    gap = 0;
                }
            }
        }
        if case < 3 {
            case
        } else if case == 3 {
            8
        } else if 3 < case && case < 8 {
            case - 1
        } else if case == 8 {
            9
        } else {
            7
        }
    }

    fn mirror_case_4e(case: usize) -> usize {
        match case {
            1 => 8,
            5 => 9,
            8 => 1,
            9 => 5,
            _ => case
        }
    }

    fn get_case_6e(shape: &Vec<usize>) -> usize {
        let highest = if let Some(max) = shape.iter().max() {*max} else {0};
        let previous = match shape.iter().position(|&x| x == highest) {
            Some(index) => {
                let prev = shape[(3 + index - 1) % 3];
                if prev == 0 && highest == 3 {
                    3
                } else {
                    prev
                }
            }
            None => 0
        };
        match highest - previous + 3.min(highest - 2) {
            0 => 3,
            1 => 4,
            2 => 0,
            3 => 7,
            4 => 1,
            5 => 5,
            6 => 8,
            7 => 2,
            8 => 9,
            9 => 6,
            _ => 3
        }
    }

    fn get_case_2e(shape: &Vec<usize>) -> usize {
        match shape.iter().position(|&x| x == 1) {
            Some(index) => {
                if shape[(5 + index - 1) % 5] == 1 || shape[(index + 1) % 5] == 1 {
                    1
                } else {
                    2
                }
            },
            None => 0
        }
    }
}