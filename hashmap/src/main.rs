use std::collections::HashMap;

fn main() {
    create_hashmap();
}

fn create_hashmap() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
}
