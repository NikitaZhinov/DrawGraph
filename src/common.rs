pub static OPERATIONS : &'static str = "+-*/^~()";
pub static FUNCTIONS : [&str; 8] = [
    "x",
    "sin", "cos", "tg", "ctg",
    "sqrt", "ln", "lg"
];

pub fn check_str_number(str_number : String) -> bool {
    for n in str_number.chars() {
        if !check_number(n) { return false; }
    }
    true
}

pub fn check_number(symbol : char) -> bool {
    if symbol.to_ascii_lowercase() >= '0'.to_ascii_lowercase() &&
        symbol.to_ascii_lowercase() <= '9'.to_ascii_lowercase() {
        return true;
    }
    false
}

pub fn check_operation(symbol : char) -> bool {
    for op in OPERATIONS.chars() {
        if symbol == op { return true; }
    }
    false
}

pub fn check_function(str_function : String) -> bool {
    for func in FUNCTIONS {
        if str_function == func { return true; }
    }
    false
}

pub struct Stack<T> {
    date: Vec<T>
}

impl<T> Stack<T> {
    fn push(&mut self, elem: T) { self.date.push(elem); }

    pub fn pop(&mut self) -> Option<T> { self.date.pop() }

    pub fn peek(&self) -> Option<&T> { self.date.last() }
}
