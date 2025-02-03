
use crate::square1::Square1;

use super::State;
use super::symmetry::Symmetry;

pub struct StateSqSq {
    sq1: Square1,
    co: usize,
    cp_black: usize,
    cp_white: usize,
    ep: usize,
    index: usize,
    up_alignment: usize,
    down_alignment: usize
}

impl State for StateSqSq {
    const NAME: &str = "sqsq";

    const SIZE: usize = 3_628_800;

    const MAX_SLICES: u8 = 9;

    fn new(sq1: Square1) -> Self {
        let up_alignment: usize = (sq1.pieces[0] & 1) as usize;
        let down_alignment: usize = (sq1.pieces[8] & 1) as usize;
        let mut state: Self = Self {sq1, co: 0, cp_black: 0, cp_white: 0, ep: 0, index: 0, up_alignment, down_alignment};
        state.calc_index();
        state
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_symmetric_indecies(&mut self) -> Vec<usize> {
        let sq1 = self.sq1.clone();
        get_symmetric_actions(self.co).iter().map(|action| {
            if action.flip_l {
                self.flip_layers();
            }
            if action.flip_c {
                self.flip_colors();
            }
            if action.mirror {
                self.mirror_layers();
            }
            self.rotate_layers(action.up_rot, action.down_rot);
            self.cp_black = 0;
            self.cp_white = 0;
            self.ep = 0;
            self.calc_permutation();
            self.combine_to_index();
            self.sq1 = sq1.clone();
            self.index
        }).collect()
    }

    fn get_square1_num(&self) -> u64 {
        self.sq1.get_num()
    }

    fn gen_next_positions(sq1num: u64) -> Vec<u64> {
        let base: Square1 = Square1::from_num(sq1num);
        base.get_unique_turns_sqsq().map(|turn: (usize, usize)| {
            let mut adj: Square1 = base.clone();
            adj.turn_layers(&turn);
            adj.turn_slice().expect("Unique Turns is wrong");
            adj.get_num()
        }).to_vec()
    }
}

impl StateSqSq {
    fn calc_index(&mut self) {
        self.calc_orientation();
        self.calc_permutation();
        self.combine_to_index();
    }

    fn combine_to_index(&mut self) {
        self.index = self.ep;
        self.index = self.index * 5 + self.co;
        self.index = self.index * 6 + self.cp_black;
        self.index = self.index * 6 + self.cp_white;
    }

    fn calc_orientation(&mut self) {
        let black: bool = self.get_corner(0) < 8;
        let mut count: usize = 1;
        for i in 1..4 {
            if (self.get_corner(i) < 8) == black {
                count += 1;
            }
        }
        match count  {
            1 => {
                self.co = 1;
                if black {
                    self.flip_layers();
                }
            }
            2 => {
                self.co = 2;
                let mut up_is_opp: bool = false;
                if (self.get_corner(2) < 8) == black {
                    self.co += 1;
                    up_is_opp = true;
                }
                if (self.get_corner(4) < 8) == (self.get_corner(6) < 8) {
                    self.co += 1;
                }
                if self.co == 3 && up_is_opp {
                    self.flip_layers();
                }
            }
            3 => {
                self.co = 1;
                if !black {
                    self.flip_layers();
                }
            }
            _ => {
                self.co = 0;
                if !black {
                    self.flip_layers();
                }
            }
        }

        let mut up_rot: usize = 0;
        let mut down_rot: usize = 0;
        for i in 0..4 {
            if self.get_corner(i) > 7 {
                up_rot = i;
            }
            if self.get_corner(4 + i) < 8 {
                down_rot = i;
            }
        }

        match self.co {
            2 => {
                if up_rot == 3 && self.get_corner(0) > 7 {
                    up_rot = 0;
                }
                if down_rot == 3 && self.get_corner(4) < 8 {
                    down_rot = 0;
                }
            }
            3 => {
                if up_rot == 3 && self.get_corner(0) > 7 {
                    up_rot = 0;
                }
            }
            _ => {}
        }

        self.rotate_layers(up_rot, down_rot);
    }

    fn calc_permutation(&mut self) {
        let mut black_offset: Option<u8> = None;
        let mut white_offset: Option<u8> = None;
        let mut index: usize = 0;
        while black_offset == None || white_offset == None {
            let corner: u8 = self.get_corner(index) / 2;
            if corner < 4 {
                if black_offset == None {
                    black_offset = Some(corner);
                }
            } else {
                if white_offset == None {
                    white_offset = Some(corner - 4)
                }
            }
            index += 1;
        }
        self.sq1.cycle_colors(&(black_offset.unwrap(), white_offset.unwrap()));

        let mut blacks: Vec<u8> = vec![];
        let mut whites: Vec<u8> = vec![];
        for i in 0..8 {
            let corner = self.get_corner(i);
            if corner < 8 {
                blacks.push(corner);
            } else {
                whites.push(corner);
            }
        }
        let mut factor: usize = 1;
        for i in 2..4 {
            factor *= i - 1;
            let mut blacks_higher: usize = 0;
            let mut whites_higher: usize = 0;
            let blacki: u8 = blacks[i];
            let whitei: u8 = whites[i];
            for j in 1..i {
                if blacks[j] > blacki {
                    blacks_higher += 1;
                }
                if whites[j] > whitei {
                    whites_higher += 1;
                }
            }
            self.cp_black += blacks_higher * factor;
            self.cp_white += whites_higher * factor;
        }
        factor = 1;
        for i in 1..8 {
            factor *= i;
            let mut edges_higher: usize = 0;
            let edgei: u8 = self.get_edge(i);
            for j in 0..i {
                if self.get_edge(j) > edgei {
                    edges_higher += 1;
                }
            }
            self.ep += edges_higher * factor;
        }
        self.ep /= 2;
    }

    fn get_corner(&self, corner_index: usize) -> u8 {
        if corner_index < 4 {
            self.sq1.pieces[self.up_alignment + corner_index * 2]
        } else {
            self.sq1.pieces[self.down_alignment + corner_index * 2]
        }
    }

    fn get_edge(&self, edge_index: usize) -> u8 {
        if edge_index < 4 {
            self.sq1.pieces[1 - self.up_alignment + edge_index * 2]
        } else {
            self.sq1.pieces[1 - self.down_alignment + edge_index * 2]
        }
    }

    fn flip_layers(&mut self) {
        self.sq1.flip_layers();
        (self.up_alignment, self.down_alignment) = (1 - self.down_alignment, 1 - self.up_alignment);
    }

    fn rotate_layers(&mut self, up_rot: usize, down_rot: usize) {
        self.sq1.turn_layers(&(self.up_alignment + up_rot * 2, self.down_alignment + down_rot * 2));
        self.up_alignment = 0;
        self.down_alignment = 0;
    }

    fn flip_colors(&mut self) {
        self.sq1.flip_colors();
    }

    fn mirror_layers(&mut self) {
        self.sq1.mirror_layers_atd(8);
        self.up_alignment = 1 - self.up_alignment;
        self.down_alignment = 1 - self.down_alignment;
    }

    
}

const fn get_symmetric_actions(co: usize) -> &'static [Symmetry] {
    match co {
        0 => CO_0,
        1 => CO_1,
        2 => CO_2,
        3 => CO_3,
        4 => CO_4,
        _ => &[]
    }
}

const CO_0: &[Symmetry]  = &[
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 0, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 0, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 1},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 0, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 0, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 3},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 1, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 1, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 0},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 1, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 1, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 2},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 2, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 1},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 2, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 2, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 3},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 2, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 3, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 3, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 0},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 3, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 3, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 2},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 3}
];

const CO_1: &[Symmetry] = &[
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 3}
];

const CO_2: &[Symmetry] = &[
    Symmetry {flip_l: true, flip_c: false, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: true, up_rot: 2, down_rot: 2}
];

const CO_3: &[Symmetry] = &[
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 2, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 2, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 0, down_rot: 3}
];

const CO_4: &[Symmetry] = &[
    Symmetry {flip_l: true, flip_c: false, mirror: false, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: true, up_rot: 1, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: false, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 0, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: true, up_rot: 1, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: true, flip_c: false, mirror: false, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 0},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: true, flip_c: true, mirror: true, up_rot: 3, down_rot: 1},
    Symmetry {flip_l: false, flip_c: false, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: true, flip_c: false, mirror: false, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: false, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: false, flip_c: false, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: true, flip_c: true, mirror: false, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: true, flip_c: false, mirror: true, up_rot: 2, down_rot: 2},
    Symmetry {flip_l: false, flip_c: true, mirror: true, up_rot: 3, down_rot: 3},
    Symmetry {flip_l: true, flip_c: true, mirror: true, up_rot: 3, down_rot: 3}
];