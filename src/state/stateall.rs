use crate::square1::Square1;

use super::State;

pub struct StateAll {
    sq1: Square1,
    cubeshape: usize,
    co: usize,
    cp_black: usize,
    cp_white: usize,
    ep: usize,
    index: usize,
    up_case: usize,
    down_case: i8,
    up_re: usize,
    down_re: usize
}

impl State for StateAll {
    const NAME: &str = "all";

    const SIZE: usize = 3_302_208_000;

    const MAX_SLICES: u8 = 12;

    fn new(sq1: Square1) -> Self {
        let mut state: Self = Self {sq1, cubeshape: 0, co: 0, cp_black: 0, cp_white: 0, ep: 0, index: 0, up_case: 0, down_case: 0, up_re: 0, down_re: 0};
        state.calc_index();
        state
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_symmetric_indecies(&mut self) -> Vec<usize> {
        let base = self.sq1.clone();
        let (flip_mirrors, up_turns, down_turns) = get_symmetric_actions(self.up_case, self.down_case);
        let mut indecies = vec![];
        for (flip, mirror) in flip_mirrors {
            if *mirror {
                self.sq1.mirror_layers();
            }
            if *flip {
                self.sq1.flip_layers();
            }
            let modified = self.sq1.clone();
            for up_turn in up_turns {
                for down_turn in down_turns {
                    if *mirror || *flip || *up_turn != 0 || *down_turn != 0 {
                        if *mirror {
                           self.sq1.turn_layers(&(self.up_re + *up_turn, self.down_re + *down_turn)); 
                        } else {
                            self.sq1.turn_layers(&(*up_turn, *down_turn));
                        }
                        self.calc_orientation();
                        self.calc_permutation();
                        self.combine_to_index();
                        self.sq1 = modified.clone();
                        indecies.push(self.index);
                    }
                }
            }
            self.sq1 = base.clone();
        }
        indecies
    }

    fn get_square1_num(&self) -> u64 {
        self.sq1.get_num()
    }

    fn gen_next_positions(sq1num: u64) -> Vec<u64> {
        let base: Square1 = Square1::from_num(sq1num);
        base.get_unique_turns().into_iter().map(|turn: (usize, usize)| {
            let mut adj: Square1 = base.clone();
            adj.turn_layers(&turn);
            adj.turn_slice().expect("Unique Turns is wrong");
            adj.get_num()
        }).collect()
    }
}

impl StateAll {
    fn calc_index(&mut self) {
        self.calc_cubeshape();
        self.calc_orientation();
        self.calc_permutation();
        self.combine_to_index();
    }

    fn combine_to_index(&mut self) {
        self.index = self.ep;
        self.index = self.index * 65 + self.cubeshape;
        self.index = self.index * 35 + self.co;
        self.index = self.index * 6 + self.cp_black;
        self.index = self.index * 6 + self.cp_white;
    }

    fn calc_cubeshape(&mut self) {
        let mut up_shape = self.get_shape(true);
        let mut down_shape = self.get_shape(false);
        if up_shape.len() > 4 {
            self.sq1.flip_layers();
            (up_shape, down_shape) = (down_shape, up_shape);
        }
        match 4 - up_shape.len() {
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

                self.up_re = if up_case != 4 {
                    Self::max(&up_shape)
                } else {
                    5
                };
                self.down_re = if down_case == 0 && up_case != 8 {
                    1
                } else if down_case == 3 {
                    2
                } else if down_case != 4 {
                    8 - Self::max(&down_shape)
                } else {
                    8 - 5
                };

                self.cubeshape = Self::sum_to(up_case) + down_case;
                self.up_case = up_case;
                self.down_case = down_case as i8;
            },
            1 => {
                let mut up_case = Self::get_case_6e(&up_shape);
                let down_case = Self::get_case_2e(&down_shape);
                if up_case > 6 {
                    self.sq1.mirror_layers_atd(9);
                    up_case -= 7;
                }

                self.up_re = if up_case != 4 {
                    Self::max(&up_shape)
                } else {
                    7
                };
                self.down_re = if down_case == 0 {
                    7 - 2
                } else if down_case == 1 {
                    7 - 3
                } else {
                    7 - 4
                };

                self.cubeshape = 39 + 3 * up_case + down_case;

                self.up_case = up_case;
                self.down_case = -1;
            },
            2 => {
                self.up_case = Self::min(&up_shape);
                self.down_case = -2;
                self.up_re = Self::max(&up_shape);
                self.down_re = 0;
                self.cubeshape = 60 + self.up_case;

            },
            _ => {}
        }
        self.correct_layers(up_shape, down_shape);
        
    }

    fn calc_orientation(&mut self) {
        if self.sq1.pieces[0] > 7 {
            self.sq1.flip_colors();
        }

        let mut corner_gaps = vec![];
        let mut gap = 0;
        let mut black_offset: Option<u8> = None;
        let mut white_offset: Option<u8> = None;
        self.sq1.pieces.iter().for_each(|&piece| {
            if piece & 1 == 0 {
                if piece > 7 {
                    gap += 1;
                    if white_offset == None {
                        white_offset = Some(piece / 2 - 4)
                    }
                } else {
                    corner_gaps.push(gap);
                    gap = 0;
                    if black_offset == None {
                        black_offset = Some(piece / 2)
                    }
                }
            }
        });
        let n1 = 4 - corner_gaps[1];
        let n2 = n1 - corner_gaps[2];
        let n3 = n2 - corner_gaps[3];
        self.co = Self::sum_sum_to(n1) + Self::sum_to(n2) + n3;

        self.sq1.cycle_colors(&(black_offset.unwrap(), white_offset.unwrap()));
    }

    fn calc_permutation(&mut self) {
        self.cp_black = 0;
        self.cp_white = 0;
        self.ep = 0;
        let mut blacks: Vec<u8> = vec![];
        let mut whites: Vec<u8> = vec![];
        let mut edges: Vec<u8> = vec![];
        let mut black_index: i32 = -1;
        let mut white_index: i32 = -1;
        let mut edge_index: usize = 0;
        let mut black_factor: usize = 1;
        let mut white_factor: usize = 1;
        let mut edge_factor: usize = 1;
        self.sq1.pieces.iter().for_each(|&piece| {
            if piece & 1 == 1 {
                if edge_index > 0 {
                    edge_factor *= edge_index;
                    let higher = edges.iter().filter(|&&check| check > piece).count();
                    self.ep += higher * edge_factor;
                }
                edges.push(piece);
                edge_index += 1;
            } else if piece > 7 {
                if white_index > 0 {
                    white_factor *= white_index as usize;
                    let higher = whites.iter().filter(|&&check| check > piece).count();
                    self.cp_white += higher * white_factor;
                }
                if white_index > -1 {
                    whites.push(piece);
                }
                white_index += 1;
            } else {
                if black_index > 0 {
                    black_factor *= black_index as usize;
                    let higher = blacks.iter().filter(|&&check| check > piece).count();
                    self.cp_black += higher * black_factor;
                }
                if black_index > -1 {
                    blacks.push(piece);
                }
                black_index += 1;
            }
        });
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

    fn correct_layers(&mut self, up_shape: Vec<usize>, down_shape: Vec<usize>) {
        self.sq1.turn_layers(&(self.get_shape_turn(true, up_shape), self.get_shape_turn(false, down_shape)));
    }

    fn get_shape_turn(&self, for_up: bool, shape: Vec<usize>) -> usize {
        let highest: usize = Self::max(&shape);
        if highest == 0 {
            0
        } else {
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
    }

    fn get_case_4e(shape: &Vec<usize>) -> usize {
        let mut case: usize = Self::max(shape);
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
        let highest = Self::max(shape);
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

    fn sum_to(val: usize) -> usize {
        (val * (val + 1)) / 2
    }

    fn sum_sum_to(val: usize) -> usize {
        if val < 1 {
            0
        } else if val < 3 {
            val * val
        } else {
            val * val + Self::sum_sum_to(val - 2)
        }
    }

    fn min(list: &Vec<usize>) -> usize {
        if let Some(min) = list.iter().min() {*min} else {0}
    }

    fn max(list: &Vec<usize>) -> usize {
        if let Some(max) = list.iter().max() {*max} else {0}
    }
}

const fn get_symmetric_actions(up_case: usize, down_case: i8) -> (&'static [(bool, bool)], &'static [usize], &'static [usize]) {
    match down_case {
        -2 => {
            (NFM, if up_case == 4 {R_2_5} else {R_1_0}, R_6_1)
        }
        -1 => {
            (if up_case > 2 {NFM} else {NAN}, if up_case == 3 {R_3_3} else {R_1_0}, R_1_0)
        }
        _ => {
            let flip_mirrors = if up_case == 8 {
                if down_case == 1 {
                    NAN
                } else {
                    FAM
                }
            } else if up_case == down_case as usize {
                if up_case == 1 || up_case == 5 {
                    FNM
                } else {
                    FOM
                }
            } else if up_case == 1 || down_case == 1 || up_case == 5 || down_case == 5 {
                NAN
            } else {
                NFM
            };
            (
                flip_mirrors,
                if up_case == 0 {R_4_2} else if up_case == 3 {R_2_4} else {R_1_0},
                if down_case == 0 && up_case != 8 {R_4_2} else if down_case == 3 {R_2_4} else {R_1_0}
            )
        }
    }
}

const FNM: &[(bool, bool)] = &[(false, false), (true, false)];
const NFM: &[(bool, bool)] = &[(false, false), (false, true)];
const FOM: &[(bool, bool)] = &[(false, false), (false, true), (true, false), (true, true)];
const FAM: &[(bool, bool)] = &[(false, false), (true, true)];
const NAN: &[(bool, bool)] = &[(false, false)];

const R_4_2: &[usize] = &[0, 2, 4, 6];
const R_2_4: &[usize] = &[0, 4];
const R_3_3: &[usize] = &[0, 3, 6];
const R_2_5: &[usize] = &[0, 5];
const R_6_1: &[usize] = &[0, 1, 2, 3, 4, 5];
const R_1_0: &[usize] = &[0];
