use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};

#[wasm_bindgen]
pub struct HTMLDivElement {
    id: u32,
    children: Vec<HTMLSpanElement>
}

pub struct HTMLSpanElement {
    id: u32,
    content: String
}

impl HTMLDivElement {
    pub fn new(id: u32) -> HTMLDivElement {
        let el = HTMLSpanElement {
            id: id + 1,
            content: String::from("Test")
        };

        HTMLDivElement {
            id,
            children: vec![el]
        }
    }

    pub fn get_children(&self) -> Vec<HTMLSpanElement> {
        self.children.clone()
    }

    pub fn get_children_count(&self) -> usize {
        self.children.len()
    }
}

// impl FromWasmAbi for HTMLSpanElement {
//     fn from_abi(&self) -> u32 {
//         100
//     }
// }

impl IntoWasmAbi for HTMLSpanElement {
    fn into_abi(&self) -> u32 {
        100
    }
}
