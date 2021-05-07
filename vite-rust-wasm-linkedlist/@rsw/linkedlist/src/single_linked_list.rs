use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
extern crate js_sys;

type CoreNode = Rc<RefCell<Node>>;
type LinkNode = Option<CoreNode>;

#[wasm_bindgen]
#[derive(Debug)]
pub struct LinkedList {
    pub length: i32,
    head: LinkNode,
    tail: LinkNode,
}

#[wasm_bindgen]
impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn to_vec(&mut self) -> js_sys::Uint32Array{
        let mut res: Vec<i32> = Vec::new();
        let head = self.head.clone().take();
        loop {
            match head {
                Some(list_node) => {
                    res.push(list_node);
                    head = list_node.next;
                }

                None => {
                    break;
                }
            }
        }
        js_sys::Uint32Array::from(&res[..])
    }

    pub fn to_array(&mut self) -> js_sys::Uint32Array {
        let result = vec![10, 20];
        return js_sys::Uint32Array::from(&result[..]);
    }

    pub fn append(&mut self, data: i32) {
        let node = Node::new(data);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(node.clone()),
            None => self.head = Some(node.clone()),
        };
        self.length += 1;
        self.tail = Some(node);
    }

    pub fn prepend(&mut self, data: i32) {
        let node = Node::new(data);
        node.borrow_mut().next = self.head.take();
        match self.tail.take() {
            Some(x) => self.tail = Some(x),
            None => self.tail = Some(node.clone()),
        };
        self.length += 1;
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .data
        })
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    data: i32,
    next: LinkNode,
}

impl Node {
    pub fn new(value: i32) -> CoreNode {
        Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        }))
    }
}
