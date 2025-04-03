use std::{fs::{self, File, OpenOptions}, io::{Read, Seek, Write}, path::PathBuf};
use bytemuck::cast_slice;

const BUFFER_SIZE: usize = 100_000_000;

pub struct PosTable {
    index: usize,
    path: PathBuf,
    file: File,
    buffer: Vec<u8>
}

impl PosTable {
    pub fn new(name: &str, slice_depth: u8) -> Self {
        let _ = fs::create_dir("temp");
        let path: PathBuf = format!("temp/{}_{}.bin", name, slice_depth).into();
        let file: File = OpenOptions::new().read(true).append(true).create(true).open(&path).expect("Failed creating file");

        Self {
            index: 0,
            path,
            file,
            buffer: vec![]
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub(super) fn len(&self) -> usize {
        self.index
    }

    pub(super) fn start_read(&mut self) {
        self.file.seek(std::io::SeekFrom::Start(0)).expect("Failed to switch to read mode");
        self.buffer = vec![0; BUFFER_SIZE]
    }

    pub(super) fn finish_read(&mut self) {
        self.buffer.clear();
        let _ = fs::remove_file(&self.path);
    }

    pub(super) fn read_chunk(&mut self) -> Option<&[u64]> {
        let mut bytes_read = 0;
        loop {
            match self.file.read(&mut self.buffer[bytes_read..]).expect("Failed to read chunk from file") {
                0 => if bytes_read == 0 {
                    return None;
                } else {
                    return Some(cast_slice(&self.buffer[..bytes_read]));
                },
                n => bytes_read += n
            }
        }
    }

    pub(super) fn write_chunk(&mut self, chunk: &[u64]) {
        self.index += chunk.len();
        self.file.write_all(cast_slice(chunk)).expect("Failed to write chunk to file");
    }
}