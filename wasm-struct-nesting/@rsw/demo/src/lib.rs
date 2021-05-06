use wasm_bindgen::prelude::*;
use std::marker::Copy;

#[wasm_bindgen]
// #[derive(Clone, Copy)]
pub struct Counter {
    val: i32,
    children: Option<Box<Counter>>
}

#[wasm_bindgen]
impl Counter {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter {
            val: 0,
            children: None,
        }
    }

    pub fn add(&mut self) -> i32 {
        self.val += 1;
        if let Some(mut children) = self.children {
            children.add();
        }

        self.val
    }

    pub fn set_children(&mut self, children: Counter) {
        self.children = children
    }

    pub fn get_children(&self) -> Option<Counter> {
        self.children
    }
}
