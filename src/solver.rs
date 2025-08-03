use std::{cmp::{max, min}, fmt, iter::Peekable, str::FromStr, sync::LazyLock};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{square1::Square1, state::{stateall::StateAll, State}, table::SliceCountTable};

static TABLE: LazyLock<Vec<u8>> = std::sync::LazyLock::new(|| SliceCountTable::<StateAll>::read_table_from_file());

pub fn load_table() {
    SliceCountTable::<StateAll>::read(&TABLE, 0);
}

fn get_slice_count(square1: Square1) -> u8 {
    let index = StateAll::new(square1).get_index();
    SliceCountTable::<StateAll>::read(&TABLE, index)
}

struct Step {
    readable: (i8, i8),
    next_steps: Vec<Self>,
    slices: i8
}

impl Step {
    fn new(readable: (i8, i8), square1: Square1, slices: i8) -> Self {
        let next_steps: Vec<Self> = if slices > 0 {
            let turns: Vec<(usize, usize)> = if slices > 1 {square1.get_unique_turns()} else {square1.get_all_turns()};
            turns.into_par_iter().filter_map(|turn| {
                let mut adj = square1.clone();
                adj.turn_layers(&turn);
                adj.turn_slice().expect("Couldn't turn slice");
                if get_slice_count(adj.clone()) < slices as u8 {
                    let next_step = Self::new(square1.get_human_readable(turn), adj, slices - 1);
                    if slices == 1 {
                        if next_step.next_steps.is_empty() {
                            None
                        } else {
                            Some(next_step)
                        }
                    } else {
                        Some(next_step)
                    }
                } else {
                    None
                }
            }).collect()
        } else if slices == 0 {
            if square1.pieces[0] < 8 {
                let abf_turn: (usize, usize) = (8 - square1.pieces[0] as usize, 16 - square1.pieces[8] as usize);
                let mut adj = square1.clone();
                adj.turn_layers(&abf_turn);
                vec![Self::new(square1.get_human_readable(abf_turn), adj, -1)]
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        Self {
            readable,
            next_steps,
            slices
        }
    }

    fn get_best_path(&self) -> Vec<(i8, i8)> {
        let (_, _, length, mut path) = self.get_best_path_raw(6, 6);
        println!("Turn Value: {}", length);
        path.pop();
        path.reverse();
        path
    }

    fn get_best_path_raw(&self, mut pot_swap_odd: u8, mut pot_swap_even: u8) -> (bool, bool, u8, Vec<(i8, i8)>) {
        if self.slices == -1 {
            let length = get_length(self.readable);
            let length_swap = get_length_swap(self.readable);
            if pot_swap_odd + length_swap < length {
                (true, false, length_swap, vec![get_swap(self.readable)])
            } else {
                (false, false, length, vec![self.readable])
            }
        } else if self.slices == 0 {
            let (swap_odd, _, total_length, mut path) = self.next_steps[0].get_best_path_raw(pot_swap_odd, pot_swap_even);
            let new_readable = if swap_odd {get_onhead(self.readable)} else {self.readable};

            let length = get_length(new_readable);
            let length_swap = get_length_swap(new_readable);
            if pot_swap_even + length_swap < length {
                path.push(get_swap(new_readable));
                (swap_odd, true, total_length + length_swap, path)
            } else {
                path.push(new_readable);
                (swap_odd, false, total_length + length, path)
            }
        } else {
            let length = get_length(self.readable);
            let length_swap = get_length_swap(self.readable);
            let new_swap = length_swap - length;
            let mut swap_is_better = false;
            let odd = self.slices % 2 == 1;

            if odd {
                if new_swap <= pot_swap_odd {
                    swap_is_better = true;
                    pot_swap_odd = new_swap;
                }
            } else {
                if new_swap <= pot_swap_even {
                    swap_is_better = true;
                    pot_swap_even = new_swap;
                }
            }
            match self.next_steps.iter().map(|step| step.get_best_path_raw(pot_swap_odd, pot_swap_even)).min_by_key(|(_, _, total_length, _)| *total_length) {
                Some((best_swap_odd, best_swap_even, best_total_length, mut best_path)) => {
                    if odd {
                        let new_readable = if best_swap_even {get_onhead(self.readable)} else {self.readable};
                        if best_swap_odd && swap_is_better {
                            best_path.push(get_swap(new_readable));
                            (false, best_swap_even, best_total_length + length_swap, best_path)
                        } else {
                            best_path.push(new_readable);
                            (best_swap_odd, best_swap_even, best_total_length + length, best_path)
                        }
                    } else {
                        let new_readable = if best_swap_odd {get_onhead(self.readable)} else {self.readable};
                        if best_swap_even && swap_is_better {
                            best_path.push(get_swap(new_readable));
                            (best_swap_odd, false, best_total_length + length_swap, best_path)
                        } else {
                            best_path.push(new_readable);
                            (best_swap_odd, best_swap_even, best_total_length + length, best_path)
                        }
                    }
                }
                None => (false, false, 85, vec![])
            }
        }
    }
}

pub struct Solution {
    pub notation: Vec<(i8, i8)>
}

impl FromStr for Solution {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars()
            .filter(|x|!x.is_whitespace())
            .peekable();
        let mut solution = match c.peek() {
            Some('(') => Self::parse_tuple(c)?,
            Some('/') => {
                c.next();
                Self::parse_tuple(c)
                    .map(|mut s|{
                        s.notation.push((0, 0));
                        s
                    })?
            },
            _ => return Err(()),
        };
        solution.notation.reverse();
        Ok(solution)
    }
}

impl Solution {
    fn parse_tuple(mut c: Peekable<impl Iterator<Item = char>>) -> Result<Solution, ()> {
        if c.next() != Some('(') {
            return Err(());
        }
        let a = Self::parse_num(&mut c)?;
        if c.next() != Some(',') {
            return Err(());
        }
        let b = Self::parse_num(&mut c)?;
        if c.next() != Some(')') {
            return Err(());
        }
        Solution::parse_slash(c)
                .map(|mut s|{
                    s.notation.push((a, b));
                    s
                })
    }

    fn parse_slash(mut c: Peekable<impl Iterator<Item = char>>) -> Result<Solution, ()> {
        match c.next() {
            Some('/') => {
                if c.peek().is_none() {
                    Ok(Solution {
                        notation: vec![(0, 0)],
                    })
                } else {
                    Self::parse_tuple(c)   
                }
            },
            None => Ok(Solution {
                notation: vec![],
            }),
            _ => Err(())
        }
        
    }

    

    fn parse_num<'a>(c: &mut Peekable<impl Iterator<Item = char>>) -> Result<i8, ()> {
        match c.next() {
            Some('-') => Self::parse_num(c).map(|x|-x),
            Some(c) => i8::from_str(c.to_string().as_str()).map_err(|_|()),
            _ => Err(())
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sequence = String::new();
        if self.notation[0] != (0, 0) || self.notation.len() < 2 {
            sequence.push_str(&format!(" ({},{})", self.notation[0].0, self.notation[0].1));
        }
        for i in 1..self.notation.len()-1 {
            sequence.push('/');
            sequence.push_str(&format!(" ({},{})", self.notation[i].0, self.notation[i].1));
        }
        if self.notation.len() > 1 {
            sequence.push('/');
            if self.notation[self.notation.len() - 1] != (0, 0) {
                sequence.push_str(&format!(" ({},{})", self.notation[self.notation.len() - 1].0, self.notation[self.notation.len() - 1].1));
            }
        }
        write!(f, "{}", sequence)
    }
}

impl Solution {
    pub fn inverse(&self) -> Self {
        let mut notation = self.notation.clone();
        notation.reverse();
        Self { notation: notation.iter().map(|(u, d)| (-u, -d)).collect() }
    }
}

pub fn solve(square1: Square1, bar_solved: bool) -> Result<Solution, ()> {
    if !square1.is_valid() {
        Err(())
    } else {
        let mut slices = get_slice_count(square1.clone());
        if (slices % 2 == 0) == bar_solved {
            if slices == 0 && square1.pieces[0] > 7 {
                slices = 2;
            }
        } else if slices == 0 {
            slices = 3;
        } else {
            slices += 1;
        }
        println!("Solvable in {} slices", slices);
        let origin = Step::new((0, 0), square1, slices as i8);
        Ok(Solution{ notation: origin.get_best_path() })
    }
}

fn get_onhead(readable: (i8, i8)) -> (i8, i8) {
    (readable.1, readable.0)
}

fn get_swap(readable: (i8, i8)) -> (i8, i8) {
    let mut up = (readable.0 + 6) % 12;
    let mut down = (readable.1 + 6) % 12;
    if up > 6 {
        up -= 12;
    }
    if down > 6 {
        down -= 12;
    }
    (up, down)
}

fn get_length(readable: (i8, i8)) -> u8 {
    max(readable.0.abs(), readable.1.abs()) as u8
}

fn get_length_swap(readable: (i8, i8)) -> u8 {
    6 - min(readable.0.abs(), readable.1.abs()) as u8
}

mod test {
    use std::str::FromStr;

    use crate::solver::Solution;

    #[test]
    pub fn test_parse() {
        let s = Solution::from_str("(2,3)/(-2,5)/(6,0)/(0,1)").unwrap();
        println!("{s}");
    }
}