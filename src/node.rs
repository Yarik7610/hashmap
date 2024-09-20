use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Node<V>
where
    V: Copy + Clone + Debug,
{
    pub key: String,
    pub val: V,
    pub next: Option<Box<Node<V>>>,
}

impl<V> Node<V>
where
    V: Copy + Clone + Debug,
{
    pub fn new(key: &str, val: V) -> Self {
        Self {
            key: key.to_string(),
            val,
            next: None,
        }
    }
}
