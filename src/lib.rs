use hashmap::HashMap;

mod hashers;
mod hashmap;
mod node;

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", 5);
}
