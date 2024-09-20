use std::{array, fmt::Debug};

use crate::{
    hashers::{easy_hasher::EasyHasher, Hasher},
    node::Node,
};

const FILL_FACTOR: f64 = 0.75;
const DEFAULT_MAX_SIZE: usize = 32;

#[derive(Debug)]
pub struct HashMap<V>
where
    V: Copy + Clone + Debug,
{
    pub values: Vec<Option<Node<V>>>,
    len: usize,
    hasher: Box<dyn Hasher>,
}

impl<V> HashMap<V>
where
    V: Copy + Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            values: vec![None; DEFAULT_MAX_SIZE],
            len: 0,
            hasher: Box::new(EasyHasher {}),
        }
    }
    pub fn insert(&mut self, key: &str, val: V) {
        if self.contains_key(key) {
            self.update_key(key, val);
        } else {
            self.len += 1;
            if self.should_resize() {
                self.resize();
            }
            let key_index = self.get_hasher_index(key);
            match self.values.get_mut(key_index) {
                Some(Some(node)) => {
                    let mut cur = node;
                    // ref means take next by ref, not own it
                    while let Some(ref mut next) = cur.next {
                        cur = next;
                    }
                    cur.next = Some(Box::new(Node::new(key, val)));
                }
                // Some(None) for upper case
                Some(None) | None => {
                    let node = Node::new(key, val);
                    self.values[key_index] = Some(node);
                }
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.values.len()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, key: &str) -> Option<V> {
        let key_index = self.get_hasher_index(key);

        match self.values.get(key_index) {
            Some(Some(node)) => {
                if node.key == key {
                    return Some(node.val.clone());
                }
                let mut cur = node;
                while let Some(ref next) = cur.next {
                    if next.key == key {
                        return Some(next.val.clone());
                    } else {
                        cur = next;
                    }
                }
                None
            }
            Some(None) | None => return None,
        }
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        let key_index = self.get_hasher_index(key);

        match self.values.get_mut(key_index) {
            Some(Some(node)) => {
                if node.key == key {
                    let removed_value = node.val.clone();
                    //deref box for first node in values cell
                    self.values[key_index] = node.next.take().map(|n| *n);
                    self.len -= 1;
                    return Some(removed_value);
                }
                let mut cur = node;
                while let Some(mut next) = cur.next.take() {
                    if next.key == key {
                        let removed_value = next.val.clone();
                        cur.next = next.next.take();
                        self.len -= 1;
                        return Some(removed_value);
                    }
                    cur.next = Some(next);
                    cur = cur.next.as_mut().unwrap();
                }
                None
            }
            Some(None) | None => None,
        }
    }
    pub fn clear(&mut self) {
        self.values = vec![None; DEFAULT_MAX_SIZE];
        self.len = 0;
    }
    pub fn contains_key(&self, key: &str) -> bool {
        let val = self.get(key);
        if let Some(_) = val {
            return true;
        }
        false
    }
    fn update_key(&mut self, key: &str, val: V) {
        let key_index = self.get_hasher_index(key);
        let node = self.values[key_index].as_mut().unwrap();
        if node.key == key {
            node.val = val;
            return;
        }
        let mut cur = node;
        while let Some(ref mut next) = cur.next {
            if next.key == key {
                next.val = val;
                return;
            }
            cur = next;
        }
    }
    fn should_resize(&self) -> bool {
        if (self.len as f64 / self.capacity() as f64) < FILL_FACTOR {
            return false;
        }
        true
    }
    fn resize(&mut self) {
        let new_capacity = self.capacity() * 2;
        let new_values: Vec<Option<Node<V>>> = vec![None; new_capacity];
        //cool trick to change hash_map.values in place and use it's methods and have access to old values too
        let old_values = std::mem::replace(&mut self.values, new_values);
        self.len = 0;
        //flatten None doesnt include it, Some return it's value
        for node in old_values.into_iter().flatten() {
            self.insert(&node.key, node.val);
            let mut cur = node;
            while let Some(next) = cur.next.take() {
                self.insert(&next.key, next.val);
                cur = *next;
            }
        }
    }
    fn get_hasher_index(&self, key: &str) -> usize {
        let hashed_key = self.hasher.get_hash(key);
        self.hasher.get_index(hashed_key, self.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;
    use crate::hashmap::DEFAULT_MAX_SIZE;
    #[test]
    fn create() {
        let hash_map: HashMap<i32> = HashMap::new();
        assert_eq!(0, hash_map.len());
        assert_eq!(DEFAULT_MAX_SIZE, hash_map.capacity());
    }
    #[test]
    fn clear() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("ten", 10);
        hash_map.insert("net", 9);
        hash_map.clear();
        assert_eq!(0, hash_map.len());
        assert_eq!(32, hash_map.capacity());
    }
    #[test]
    fn insert_same_keys() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("ten", 10);
        hash_map.insert("ten", 9);
        assert_eq!(Some(9), hash_map.get("ten"));
    }
    #[test]
    fn get_different_keys() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("ten", 10);
        hash_map.insert("net", 9);
        assert_eq!(Some(10), hash_map.get("ten"));
        assert_eq!(Some(9), hash_map.get("net"));
    }
    #[test]
    fn get_non_existing_key() {
        let hash_map: HashMap<i32> = HashMap::new();
        assert_eq!(None, hash_map.get("lol"));
    }
    #[test]
    fn contains_existing_key() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("ten", 10);
        assert_eq!(true, hash_map.contains_key("ten"));
    }
    #[test]
    fn contains_non_existing_key() {
        let hash_map: HashMap<i32> = HashMap::new();
        assert_eq!(false, hash_map.contains_key("-013"));
    }
    #[test]
    fn remove_existing_key() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("net", 9);
        assert_eq!(Some(9), hash_map.remove("net"));
    }
    #[test]
    fn remove_key_twice() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("ten", 10);
        assert_eq!(Some(10), hash_map.remove("ten"));
        assert_eq!(None, hash_map.remove("ten"));
    }
    #[test]
    fn remove_non_existing_key() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        assert_eq!(None, hash_map.remove(""));
    }
    #[test]
    fn len_after_non_existing_remove() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        assert_eq!(0, hash_map.len());
        hash_map.remove("key");
        assert_eq!(0, hash_map.len());
    }
    #[test]
    fn len_after_existing_remove() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        hash_map.insert("key", 5);
        assert_eq!(1, hash_map.len());
        hash_map.remove("key");
        assert_eq!(0, hash_map.len());
    }
    #[test]
    fn capacity_before_expand() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        for i in 0..23 {
            hash_map.insert(&format!("key{i}"), i);
        }
        assert_eq!(DEFAULT_MAX_SIZE, hash_map.capacity());
    }
    #[test]
    fn capacity_after_expand() {
        let mut hash_map: HashMap<i32> = HashMap::new();
        for i in 0..24 {
            hash_map.insert(&format!("key{i}"), i);
        }
        assert_eq!(DEFAULT_MAX_SIZE * 2, hash_map.capacity());
    }
    #[test]
    fn should_resize_if_more_than_fill_factor() {
        let mut hash_map: HashMap<usize> = HashMap::new();
        for i in 0..hash_map.capacity() {
            hash_map.insert(&format!("key{i}"), i);
        }
        assert_eq!(false, hash_map.should_resize());
        assert_eq!(DEFAULT_MAX_SIZE * 2, hash_map.capacity());
    }
    #[test]
    fn should_resize_if_equals_fill_factor() {
        let mut hash_map: HashMap<usize> = HashMap::new();
        for i in 0..24 {
            hash_map.insert(&format!("key{i}"), i);
        }
        assert_eq!(DEFAULT_MAX_SIZE * 2, hash_map.capacity());
        assert_eq!(false, hash_map.should_resize());
    }
    #[test]
    fn should_resize_if_less_than_fill_factor() {
        let mut hash_map: HashMap<usize> = HashMap::new();
        for i in 0..23 {
            hash_map.insert(&format!("key{i}"), i);
        }
        assert_eq!(false, hash_map.should_resize())
    }
}
