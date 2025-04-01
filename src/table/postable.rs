
// chunk read

// chunk write

use std::{fs::{self, File, OpenOptions}, io::{Read, Seek, Write}, path::PathBuf};
use bytemuck::{cast_slice, cast_slice_mut};

pub struct PosTable {
    index: usize,
    path: PathBuf,
    file: File
}

impl PosTable {
    pub fn new(name: &str, slice_depth: u8) -> Self {
        let _ = fs::create_dir("temp");
        let path: PathBuf = format!("temp/{}_{}.bin", name, slice_depth).into();
        let file: File = OpenOptions::new().read(true).append(true).create(true).open(&path).expect("Failed creating file");

        Self {
            index: 0,
            path,
            file
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub(super) fn len(&self) -> usize {
        self.index
    }

    pub(super) fn read_mode(&mut self) {
        self.file.seek(std::io::SeekFrom::Start(0)).expect("Failed to switch to read mode");
    }

    pub(super) fn clear_file(&mut self) {
        let _ = fs::remove_file(&self.path);
    }

    pub(super) fn read_chunk(&mut self, buffer: &mut [u64]) -> Option<usize> {
        match self.file.read(cast_slice_mut(buffer)).expect("Failed to read chunk from file") {
            0 => None,
            n => Some(n / 8)
        }
    }

    pub(super) fn write_chunk(&mut self, chunk: &[u64]) {
        self.index += chunk.len();
        self.file.write_all(cast_slice(chunk)).expect("Failed to write chunk to file");
    }
}