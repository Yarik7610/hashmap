use super::Hasher;

#[derive(Debug)]
pub struct MidHasher {}

impl Hasher for MidHasher {
    fn get_hash(&self, key: &str) -> u64 {
        let mut hash = 0;
        for (i, ch) in key.chars().enumerate() {
            //Move on 31 == (hash << 5) - hash
            hash = (hash << 5) - hash + (ch as u64) * (i as u64 + 1);
        }
        hash
    }
    fn get_index(&self, hash: u64, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        (hash % len as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_hash_usual() {
        let hasher = MidHasher {};
        assert_eq!(112320, hasher.get_hash("net"));
        assert_eq!(118068, hasher.get_hash("ten"));
    }
    #[test]
    fn get_index_usual() {
        let hasher = MidHasher {};
        assert_eq!(0, hasher.get_index(112320, 32));
        assert_eq!(20, hasher.get_index(118068, 32));
    }
    #[test]
    fn get_index_usual_resized() {
        let hasher = MidHasher {};
        assert_eq!(0, hasher.get_index(112320, 64));
        assert_eq!(52, hasher.get_index(118068, 64));
    }
    #[test]
    fn get_hash_zero() {
        let hasher = MidHasher {};
        assert_eq!(0, hasher.get_hash(""));
    }
    #[test]
    fn get_index_zero() {
        let hasher = MidHasher {};
        assert_eq!(0, hasher.get_index(0, 32));
    }
}
