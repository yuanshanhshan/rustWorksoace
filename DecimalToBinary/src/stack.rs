// stack.rs
use std::vec::Vec;

#[derive(Debug)]
pub struct Stack<T> {
    top: usize, // 栈顶
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }
}
