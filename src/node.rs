use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Node<V: Copy + Clone + Debug> {
    pub key: String,
    pub val: V,
    pub next: Option<Box<Node<V>>>,
}

impl<V: Copy + Clone + Debug> Node<V> {
    pub fn new(key: &str, val: V) -> Self {
        Self {
            key: key.to_string(),
            val,
            next: None,
        }
    }
}
