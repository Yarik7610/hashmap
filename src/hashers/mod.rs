use std::fmt::Debug;

pub mod easy_hasher;
pub trait Hasher: Debug {
    fn get_hash(&self, key: &str) -> u64;
    fn get_index(&self, hash: u64, len: usize) -> usize;
}
