mod symmetry;

pub mod statecs;
pub mod statesqsq;
pub mod stateall;

use crate::square1::Square1;

pub trait State {
    const NAME: &str;
    const SIZE: usize;
    const MAX_SLICES: u8;
    fn new(sq1: Square1) -> Self;
    fn get_index(&self) -> usize;
    fn get_symmetric_indecies(&mut self) -> Vec<usize>;
    fn get_square1_num(&self) -> u64;
    fn gen_next_positions(sq1num: u64) -> Vec<u64>;
}