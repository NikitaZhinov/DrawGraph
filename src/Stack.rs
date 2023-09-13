use std::io::Take;

pub struct stack<T> {
    date: Vec<T>
}

impl<T> stack<T> {
    fn push_to_stack(&mut self, elem: T) {
        self.date.push(elem);
    }

    pub fn pop_in_stack(&mut self) -> T {
        self.date.pop()
    }

    pub fn peek_stack(&self) -> T {
        self.date.last()
    }
}
