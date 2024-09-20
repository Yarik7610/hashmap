use super::Hasher;

#[derive(Debug)]
pub struct EasyHasher {}

impl Hasher for EasyHasher {
    fn get_hash(&self, key: &str) -> u64 {
        let mut total = 0;
        for ch in key.chars() {
            total += ch as u64;
        }
        total
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
        let hasher = EasyHasher {};
        assert_eq!(315, hasher.get_hash("kek"));
    }
    #[test]
    fn get_index_usual() {
        let hasher = EasyHasher {};
        assert_eq!(27, hasher.get_index(315, 32));
    }
    #[test]
    fn get_hash_zero() {
        let hasher = EasyHasher {};
        assert_eq!(0, hasher.get_hash(""));
    }
    #[test]
    fn get_index_zero() {
        let hasher = EasyHasher {};
        assert_eq!(0, hasher.get_index(0, 32));
    }
}
