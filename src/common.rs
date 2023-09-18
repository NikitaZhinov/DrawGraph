use std::collections::HashMap;

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

pub fn check_operation(symbol : String) -> isize {
    let operations = HashMap::from([
        ("(", 0),
        (")", 0),
        ("+", 1),
        ("-", 1),
        ("*", 2),
        ("/", 2),
        ("^", 3),
        ("~", 4)
    ]);

    for (op, priority) in operations {
        if symbol == op.to_string() { return priority; }
    }
    -1
}

pub fn check_function(str_function : String) -> bool {
    for func in FUNCTIONS {
        if str_function == func { return true; }
    }
    false
}
