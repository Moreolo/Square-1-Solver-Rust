
// chunk read

// chunk write

use std::fs;
use bytemuck::cast_slice;

const MIN_CHUNK: i64 = 5000;

pub struct PosTable {
    name: String,
    table: Vec<u64>,
    index: usize,
    tab: usize,
    size: usize
}

impl PosTable {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            table: Vec::with_capacity(size),
            index: 0,
            tab: 0,
            size: size
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.tab == 0 && self.index == 0
    }

    pub(super) fn _len(&self) -> usize {
        self.tab * self.size + self.index
    }

    pub(super) fn _clear(&mut self) {
        self.table.clear();
        while self.tab > 0 {
            self.tab -= 1;
            let path = self.get_path();
            match fs::remove_file(&path) {
                Ok(_) => {},
                Err(_) => println!("Failed to remove file {}", path)
            };
        }
        self.index = 0;
    }

    pub fn read_chunk(&mut self, size: usize) -> Option<Vec<u64>> {
        if !self.is_empty() {
            let split: i64 = (self.index as i64) - (size as i64);
            if split > MIN_CHUNK {
                self.index = split as usize;
                Some(self.table.split_off(split as usize))
            } else {
                let chunk = self.table.clone();
                let _ = self.read_table_from_file();
                Some(chunk)
            }
        } else {
            None
        }
    }

    pub fn write_chunk(&mut self, chunk: &mut Vec<u64>) {
        let split: usize = self.size - self.index;
        let chunk_len: usize = chunk.len();
        if split <= chunk_len {
            let mut rest: Vec<u64> = chunk.split_off(split);
            self.table.append(chunk);
            self.write_table_to_file();
            self.write_chunk(&mut rest);
        } else {
            self.table.append(chunk);
            self.index += chunk_len;
        }
        
    }

    fn read_table_from_file(&mut self) -> Result<(), ()> {
        if self.tab > 0 {
            self.tab -= 1;
            let path = self.get_path();
            // fs::read(&path).expect(format!("Unable to read file {}", path).as_str()).chunks_exact(8).for_each(|bytes| {
            //     self.table.push(u64::from_be_bytes(bytes.try_into().unwrap()));
            // });
            cast_slice(&fs::read(&path).expect(format!("Unable to read file {}", path).as_str())).iter().for_each(|item: &u64| {
                self.table.push(*item);
            });
            //self.table = cast_slice(&fs::read(&path).expect(format!("Unable to read file {}", path).as_str())).to_vec();
            self.index = self.size;
            match fs::remove_file(&path) {
                Ok(_) => {},
                Err(_) => println!("Failed to remove file {}", path)
            };
            Ok(())
        } else {
            self.index = 0;
            Err(())
        }
    }

    fn write_table_to_file(&mut self) {
        let path = self.get_path();
        fs::write(&path, cast_slice(&self.table)).expect(format!("Unable to write file {}", path).as_str());
        self.tab += 1;
        self.index = 0;
        self.table.clear();
    }

    fn get_path(&self) -> String {
        format!("temp/{}_{}.bin", self.name, self.tab)
    }
}