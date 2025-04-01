
// chunk read

// chunk write

use std::{fs::{self, OpenOptions}, path::PathBuf};
use bytemuck::cast_slice;
use memmap2::MmapMut;

pub struct PosTable {
    table: MmapMut,
    index: usize,
    path: PathBuf
}

impl PosTable {
    pub fn new(name: &str, slice_depth: u8, size: u64) -> Self {
        let _ = fs::create_dir("temp");
        let path: PathBuf = format!("temp/{}_{:2}.bin", name, slice_depth).into();
        let file = OpenOptions::new().read(true).write(true).create(true).open(&path).expect("Failed creating file");
        file.set_len(size).expect("Failed setting length of file");

        let table = unsafe {
            MmapMut::map_mut(&file).expect("Failed creating memmap")
        };
        Self {
            table,
            index: 0,
            path
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.index == 0
    }

    pub(super) fn len(&self) -> usize {
        self.index
    }

    pub(super) fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &[u64]> {
        self.table[..self.index * 8].chunks(chunk_size).map(|chunk| cast_slice::<u8, u64>(chunk))
    }
    pub(super) fn clear_file(&mut self) {
        let _ = fs::remove_file(&self.path);
    }

    pub(super) fn write_chunk(&mut self, chunk: &[u64]) {
        let len = chunk.len();
        let src = cast_slice(chunk);
        self.table[self.index * 8 .. (self.index + len) * 8].copy_from_slice(src);
        self.index += len;
    }
}