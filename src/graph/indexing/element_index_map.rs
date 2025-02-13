use hashbrown::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use super::ElementIndex;

pub(crate) type ElementIndexMap<T> = HashMap<ElementIndex, T, BuildIndexHasher>;

pub(crate) type BuildIndexHasher = BuildHasherDefault<IndexHasher>;

#[derive(Default)]
pub(crate) struct IndexHasher {
    hash: u64,
}

// From: https://crates.io/crates/nohash-hasher
impl Hasher for IndexHasher {
    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }

    fn write_u8(&mut self, hash: u8) {
        self.hash = u64::from(hash)
    }
    fn write_u16(&mut self, hash: u16) {
        self.hash = u64::from(hash)
    }
    fn write_u32(&mut self, hash: u32) {
        self.hash = u64::from(hash)
    }
    fn write_u64(&mut self, hash: u64) {
        self.hash = hash
    }
    fn write_usize(&mut self, hash: usize) {
        self.hash = hash as u64
    }

    fn write_i8(&mut self, hash: i8) {
        self.hash = hash as u64
    }
    fn write_i16(&mut self, hash: i16) {
        self.hash = hash as u64
    }
    fn write_i32(&mut self, hash: i32) {
        self.hash = hash as u64
    }
    fn write_i64(&mut self, hash: i64) {
        self.hash = hash as u64
    }
    fn write_isize(&mut self, hash: isize) {
        self.hash = hash as u64
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}
