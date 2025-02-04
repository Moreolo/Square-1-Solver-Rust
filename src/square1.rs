use core::array::from_fn;

const SQSQ_UNIQUE_TURNS_BA: [(usize, usize); 16] = [(1, 0), (5, 0), (3, 0), (7, 0),
                                                    (0, 1), (0, 5), (2, 1), (6, 1),
                                                    (1, 2), (1, 6), (7, 2), (7, 6),
                                                    (0, 3), (0, 7), (2, 7), (6, 7)];

const SQSQ_UNIQUE_TURNS_UA: [(usize, usize); 16] = [(0, 0), (4, 0), (2, 0), (6, 0),
                                                    (1, 1), (5, 1), (3, 1), (7, 1),
                                                    (0, 2), (0, 6), (2, 2), (2, 6),
                                                    (1, 3), (1, 7), (7, 7), (7, 3)];

const SQSQ_UNIQUE_TURNS_DA: [(usize, usize); 16] = [(0, 0), (4, 0), (2, 0), (6, 0),
                                                    (1, 1), (1, 5), (3, 1), (7, 1),
                                                    (0, 2), (0, 6), (2, 2), (2, 6),
                                                    (1, 3), (1, 7), (7, 7), (3, 7)];


const _SQSQ_ALL_TURNS_A: [(usize, usize); 32] = [(1, 0), (5, 0), (3, 0), (7, 0),
                                                (1, 4), (5, 4), (3, 4), (7, 4),
                                                (0, 1), (4, 1), (2, 1), (6, 1),
                                                (0, 5), (4, 5), (2, 5), (6, 5),
                                                (1, 2), (5, 2), (3, 2), (7, 2),
                                                (1, 6), (5, 6), (3, 6), (7, 6),
                                                (0, 3), (4, 3), (2, 3), (6, 3),
                                                (0, 7), (4, 7), (2, 7), (6, 7)];

const _SQSQ_ALL_TURNS_M: [(usize, usize); 32] = [(0, 0), (4, 0), (2, 0), (6, 0),
                                                (0, 4), (4, 4), (2, 4), (6, 4),
                                                (1, 1), (5, 1), (3, 1), (7, 1),
                                                (1, 5), (5, 5), (3, 5), (7, 5),
                                                (0, 2), (4, 2), (2, 2), (6, 2),
                                                (0, 6), (4, 6), (2, 6), (6, 6),
                                                (1, 3), (5, 3), (3, 3), (7, 3),
                                                (1, 7), (5, 7), (3, 7), (7, 7)];

#[derive(Clone, Debug)]
pub struct Square1 {
    pub(crate) pieces: [u8; 16],
}

impl Square1 {
    pub fn solved() -> Square1 {
        Square1 {pieces: from_fn(|i| i as u8)}
    }

    pub(crate) fn _from_arr(arr: [u8; 16]) -> Square1 {
        Square1 {pieces: arr}
    }

    pub fn from_num(mut num: u64) -> Square1 {
        let mut arr: [u8; 16] = [0; 16];
        for i in 0..16 {
            arr[15-i] = (num & 15) as u8;
            num >>= 4;
        }
        Square1 {pieces: arr}
    }

    pub fn get_num(&self) -> u64 {
        let mut num: u64 = 0;
        for piece in self.pieces {
            num *= 16;
            num += piece as u64;
        }
        num
    }

    fn get_angle(&self, index: usize) -> u8 {
        2 - (self.pieces[index] & 1)
    }

    pub fn turn_slice(&mut self) -> Result<(), ()> {
        let mut angle: u8 = 0;
        let mut start: usize = 0;
        let mut end: usize = 16;
        while angle < 6 {
            angle += self.get_angle(start);
            start += 1;
        }
        if angle > 6 {
            return Err(())
        }
        angle = 0;
        while angle < 6 {
            angle += self.get_angle(end - 1);
            end -= 1;
        }
        if angle > 6 {
            return Err(())
        }
        self.pieces[start..end].reverse();
        Ok(())
    }

    fn get_divide(&self) -> usize {
        let mut angle: u8 = 0;
        let mut divide: usize = 0;
        while angle < 12 {
            angle += self.get_angle(divide);
            divide += 1;
        }
        divide
    }

    pub fn turn_layers(&mut self, turn: &(usize, usize)) {
        let &(up, down) = turn;
        if up != 0 || down != 0 {
            let divide: usize = self.get_divide();
            self.pieces[ ..divide].rotate_left(up);
            self.pieces[divide.. ].rotate_left(down);
        }
    }

    pub(crate) fn cycle_colors(&mut self, cycle: &(u8, u8)) {
        let &(up, down) = cycle;
        if up != 0 || down != 0 {
            for i in 0..16 {
                if self.pieces[i] < 8 {
                    self.pieces[i] = (8 + self.pieces[i] - up * 2) % 8
                } else {
                    self.pieces[i] = (self.pieces[i] - down * 2) % 8 + 8
                }
            }
        }
    }

    pub(crate) fn flip_colors(&mut self) {
        for i in 0..16 {
            if self.pieces[i] == 15 {
                self.pieces[i] = 7;
            } else {
                self.pieces[i] = 14 - self.pieces[i];
                if self.pieces[i] == 7 {
                    self.pieces[i] += 8
                }
            }
        }
    }

    pub(crate) fn flip_layers(&mut self) {
        self.pieces.reverse();
    }

    pub(crate) fn mirror_layers_atd(&mut self, divide: usize) {
        self.pieces[ ..divide].reverse();
        self.pieces[divide.. ].reverse();
        for i in 0..16 {
            if self.pieces[i] < 8 {
                self.pieces[i] = (self.pieces[i] + 4) % 8 + 8
            } else {
                self.pieces[i] = (self.pieces[i] + 4) % 8
            }
        }
    }

    pub(crate) fn mirror_layers(&mut self) {
        self.mirror_layers_atd(self.get_divide());
    }

    pub fn get_unique_turns(&self) -> Vec<(usize, usize)> {
        let mut angle: u8 = 0;
        let mut turn: usize = 0;
        let mut potential_angles: Vec<u8> = Vec::new();
        let mut potential_turns: Vec<usize> = Vec::new();

        while angle < 6 {
            potential_angles.push(angle);
            potential_turns.push(turn);
            angle += self.get_angle(turn);
            turn += 1;
        }

        let mut up_angle_turns: Vec<(u8, usize, usize)> = Vec::new();
        while angle < 12 {
            let potential_angle = angle - 6;
            match potential_angles.iter().position(|&x| x == potential_angle) {
                Some(index) => {up_angle_turns.push((potential_angle, potential_turns[index], turn))},
                None => {}
            }
            angle += self.get_angle(turn);
            turn += 1;
        }

        let divide: usize = turn;
        angle = 0;
        turn = 0;
        potential_angles.clear();
        potential_turns.clear();
        while angle < 6 {
            potential_angles.push(angle);
            potential_turns.push(turn);
            angle += self.get_angle(divide + turn);
            turn += 1;
        }
        let mut turns = Vec::new();
        while angle < 12 {
            let potential_angle = angle - 6;
            match potential_angles.iter().position(|&x| x == potential_angle) {
                Some(index) => {
                    for (up_angle, up_turn1, up_turn2) in &up_angle_turns {
                        if *up_angle + potential_angle < 7 {
                            turns.push((*up_turn1, potential_turns[index]));
                        } else {
                            turns.push((*up_turn2, turn));
                        }
                        if *up_angle < potential_angle {
                            turns.push((*up_turn1, turn));
                        } else {
                            turns.push((*up_turn2, potential_turns[index]));
                        }
                    }
                },
                None => {}
            }
            angle += self.get_angle(divide + turn);
            turn += 1;
        }
        turns
    }

    pub fn get_unique_turns_sqsq(&self) -> [(usize, usize); 16] {
        if self.pieces[0] & 1 != self.pieces[15] & 1 {
            SQSQ_UNIQUE_TURNS_BA
        } else if self.pieces[0] & 1 == 0 {
            SQSQ_UNIQUE_TURNS_UA
        } else {
            SQSQ_UNIQUE_TURNS_DA
        }
    }

    pub(crate) fn _get_all_turns_sqsq(&self) -> [(usize, usize); 32] {
        if self.pieces[0] & 1 != self.pieces[15] & 1 {
            _SQSQ_ALL_TURNS_A
        } else {
            _SQSQ_ALL_TURNS_M
        }
    }
}