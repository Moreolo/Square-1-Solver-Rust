pub mod postable;

use std::{fs::{self}, str::FromStr, sync::{Arc, RwLock}, time::{Duration, Instant}};

use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use postable::PosTable;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{square1::Square1, state::State};

pub struct SliceCountTable <S: State + Sync> {
    pb_table: ProgressBar,
    pb_closed: ProgressBar,
    _marker: std::marker::PhantomData<S>
}

impl<S: State + Sync> SliceCountTable<S> {
    pub fn new(display_progress: bool) -> Self {
        let _multipb = if display_progress {MultiProgress::new()} else {MultiProgress::with_draw_target(ProgressDrawTarget::hidden())};
        let pb_table = _multipb.add(ProgressBar::new(S::SIZE as u64));
        let pb_closed = _multipb.add(ProgressBar::new(1));

        let style_table: ProgressStyle = ProgressStyle::with_template(
            "[{elapsed_precise}] Table: {bar:40.cyan/blue} {percent_precise:>7}% {msg}"
        )
        .unwrap()
        .progress_chars("#>-");
        let style_opened = ProgressStyle::with_template(
            "  Slice Depth {msg:>2}: {bar:40.gray/white} {percent_precise:>7}%"
        )
        .unwrap()
        .progress_chars("##-");

        pb_table.set_style(style_table);
        pb_closed.set_style(style_opened);
        pb_closed.set_message(format!("{}", 0));

        Self {
            pb_table,
            pb_closed,
            _marker: std::marker::PhantomData
        }
    }

    pub fn generate(&self) {
        // Starts time measurement
        let now = Instant::now();

        // Creates empty Slice Count Table
        let shared_table: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(vec![255 as u8; (S::SIZE + 1) / 2]));

        // Creates empty closed Table
        let mut closed = vec![];

        // Fills in the solved State and adds first closed Position
        let solved = Square1::solved();
        closed.push(solved.get_num());
        let state = S::new(solved);
        let _ = Self::write_shared(&shared_table, state.get_index(), 0);
        self.pb_table.inc(1);

        // Starts looping over the Slice Depths
        let mut slice_depth = 1;
        while !closed.is_empty() && !self.table_is_full() {
            // Shows Progress
            self.clear_pb_closed(closed.len() as u64, slice_depth);

            // Iterates over all Positions in closed Table
            let at_max = slice_depth == S::MAX_SLICES - 1;
            closed = closed.into_par_iter().flat_map_iter(|curr_square1| {
                // Shows Progress
                self.pb_closed.inc(1);

                // Opens the next Positions
                S::gen_next_positions(curr_square1).into_iter().filter(|&next_square1| {
                    // Calculates the State for the Position
                    let mut state = S::new(Square1::from_num(next_square1));

                    // Tries to write State to Table
                    match Self::write_shared(&shared_table, state.get_index(), slice_depth) {
                        Ok(_) => {
                            // On write, also write symmetric Indecies
                            let inc = 1 + state.get_symmetric_indecies().into_iter().filter(|&sym_index| {
                                Self::write_shared(&shared_table, sym_index, slice_depth) == Ok(())
                            }).count();
                            // Increase Progressbar
                            self.pb_table.inc(inc as u64);
                            !at_max
                        }
                        Err(_) => false
                    }
                })
            }).collect();
            // Increases the Slice Depth
            slice_depth += 1;
        }

        // Fills rest of the Table
        if slice_depth == S::MAX_SLICES && !self.table_is_full() {
            // Shows Progress
            self.pb_table.set_message("Filling rest");
            self.clear_pb_closed(S::SIZE as u64 - self.pb_table.position(), slice_depth);

            // Iterates Table
            let mut table = shared_table.write().unwrap();
            table.par_iter_mut().for_each(|table_value| {
                // Finds empty entries
                let mut changed: u64 = 0;
                let left = {
                    let read = *table_value >> 4;
                    if read == 15 {
                        changed += 1;
                        S::MAX_SLICES
                    } else {
                        read
                    }
                };
                let right = {
                    let read = *table_value & 15;
                    if read == 15 {
                        changed += 1;
                        S::MAX_SLICES
                    } else {
                        read
                    }
                };
                // Fills empty entries
                if changed > 0 {
                    *table_value = (left << 4) + right;
                }

                // Shows progress
                self.pb_table.inc(changed);
                self.pb_closed.inc(changed);
            });
        }

        // Finishes Time measurement
        let elapsed = now.elapsed();
        // Completes Progress
        self.pb_table.finish();
        self.pb_closed.finish();
        println!("Finished generating Table in {}", format_duration(elapsed));

        // Saves Table to file
        println!("Saving Table to file");
        {
            let table = shared_table.read().unwrap();
            let _ = fs::create_dir("slice_count_tables");
            fs::write(Self::get_file_name(), table.clone()).expect("Saving Table failed!");
        }
    }

    pub fn generate_compact(&self) {
        // Starts time measurement
        let now = Instant::now();

        // Creates empty Slice Count Table
        let shared_table: Arc<RwLock<Vec<u8>>> = Arc::new(RwLock::new(vec![255 as u8; (S::SIZE + 1) / 2]));

        // Creates empty closed Table
        let mut closed = PosTable::new("closed", 0);

        // Fills in the solved State and adds first closed Position
        let solved = Square1::solved();
        closed.write_chunk(&[solved.get_num()]);

        let state = S::new(solved);
        let _ = Self::write_shared(&shared_table, state.get_index(), 0);
        self.pb_table.inc(1);

        // Starts looping over the Slice Depths
        let mut slice_depth = 1;
        while !closed.is_empty() && !self.table_is_full() {
            // Shows Progress
            self.clear_pb_closed(closed.len() as u64, slice_depth);

            // Iterates over all Positions in closed Table
            let at_max = slice_depth == S::MAX_SLICES - 1;
            let mut new_closed = PosTable::new("closed", slice_depth);
            closed.start_read();
            while let Some(chunk) = closed.read_chunk() {
                let new_chunk: Vec<u64> = chunk.into_par_iter().flat_map_iter(|curr_square1| {
                    // Shows Progress
                    self.pb_closed.inc(1);
    
                    // Opens the next Positions
                    S::gen_next_positions(*curr_square1).into_iter().filter(|&next_square1| {
                        // Calculates the State for the Position
                        let mut state = S::new(Square1::from_num(next_square1));
    
                        // Tries to write State to Table
                        match Self::write_shared(&shared_table, state.get_index(), slice_depth) {
                            Ok(_) => {
                                // On write, also write symmetric Indecies
                                let inc = 1 + state.get_symmetric_indecies().into_iter().filter(|&sym_index| {
                                    Self::write_shared(&shared_table, sym_index, slice_depth) == Ok(())
                                }).count();
                                // Increase Progressbar
                                self.pb_table.inc(inc as u64);
                                !at_max
                            }
                            Err(_) => false
                        }
                    })
                }).collect();
                new_closed.write_chunk(&new_chunk);
            }
            closed.finish_read();
            closed = new_closed;
            // Increases the Slice Depth
            slice_depth += 1;
        }

        closed.finish_read();

        // Fills rest of the Table
        if slice_depth == S::MAX_SLICES && !self.table_is_full() {
            // Shows Progress
            self.pb_table.set_message("Filling rest");
            self.clear_pb_closed(S::SIZE as u64 - self.pb_table.position(), slice_depth);

            // Iterates Table
            let mut table = shared_table.write().unwrap();
            table.par_iter_mut().for_each(|table_value| {
                // Finds empty entries
                let mut changed: u64 = 0;
                let left = {
                    let read = *table_value >> 4;
                    if read == 15 {
                        changed += 1;
                        S::MAX_SLICES
                    } else {
                        read
                    }
                };
                let right = {
                    let read = *table_value & 15;
                    if read == 15 {
                        changed += 1;
                        S::MAX_SLICES
                    } else {
                        read
                    }
                };
                // Fills empty entries
                if changed > 0 {
                    *table_value = (left << 4) + right;
                }

                // Shows progress
                self.pb_table.inc(changed);
                self.pb_closed.inc(changed);
            });
        }

        // Finishes Time measurement
        let elapsed = now.elapsed();
        // Completes Progress
        self.pb_table.finish();
        self.pb_closed.finish();
        println!("Finished generating Table in {}", format_duration(elapsed));

        // Saves Table to file
        println!("Saving Table to file");
        {
            let table = shared_table.read().unwrap();
            let _ = fs::create_dir("slice_count_tables");
            fs::write(Self::get_file_name(), table.clone()).expect("Saving Table failed!");
        }
    }

    fn table_is_full(&self) -> bool {
        self.pb_table.position() == S::SIZE as u64
    }

    // Resets the Progressbar for the closed Table
    fn clear_pb_closed(&self, closed_len: u64, slice_depth: u8) {
        self.pb_closed.set_position(0);
        self.pb_closed.set_length(closed_len);
        self.pb_closed.set_message(format!("{}", slice_depth));
    }

    // Writes a value into the index of the shared Table
    fn write_shared(shared_table: &Arc<RwLock<Vec<u8>>>, index: usize, value: u8) -> Result<(), ()> {
        if {
            // Gets Lock
            let table = shared_table.read().unwrap();
            // Checks, if entry is already filled
            let table_value = table[index >> 1];
            if index & 1 == 0 {
                (table_value >> 4) == 15
            } else {
                (table_value & 15) == 15
            }
        } {
            // Gets Lock
            let mut table = shared_table.write().unwrap();
            // Gets entry
            let table_value = table[index >> 1];
            // Writes new u4 entry onto u8 slot
            if index & 1 == 0 {
                let left = table_value >> 4;
                if left == 15 {
                    let right = table_value & 15;
                    table[index >> 1] = (value << 4) + right;
                    Ok(())
                } else {
                    Err(())
                }
            } else {
                let right = table_value & 15;
                if right == 15 {
                    let left = table_value >> 4;
                    table[index >> 1] = (left << 4) + value;
                    Ok(())
                } else {
                    Err(())
                }
            }
        } else {
            Err (())
        }
    }

    pub fn read(table: &Vec<u8>, index: usize) -> u8 {
        if index & 1 == 0 {
            table[index / 2] >> 4
        } else {
            table[index / 2] & 15
        }
    }

    pub fn read_table_from_file() -> Vec<u8> {
        println!("Loading Table");
        fs::read(Self::get_file_name()).expect("Unable to read file")
    }

    pub fn get_file_name() -> String {
        String::from_str("slice_count_tables/table_").unwrap() + S::NAME + ".bin"
    }
}

pub fn format_duration(dur: Duration) -> String {
    let secs = dur.as_secs();
    let minutes = secs / 60;
    if minutes > 60 {
        format!("{}h {:2}min", minutes / 60, minutes & 60)
    } else if minutes > 0 {
        format!("{}min {:2}s", minutes, secs & 60)
    } else {
        format!("{:.2?}", dur)
    }
    // 1h 45min 34s
}