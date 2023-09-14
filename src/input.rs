use std::io;
use crate::common::*;

pub fn get_expression(tokens: &mut Vec<String>) -> bool {
    // input expression
    let mut expression: String = String::new();
    return match io::stdin().read_line(&mut expression) {
        Ok(_) => {
            // parsing expression
            parsing(tokens, expression);

            check(tokens.clone())
        },
        Err(_) => { false }
    }
}

fn parsing(tokens: &mut Vec<String>, expression: String) {
    let mut i = 0;
    while i < expression.len() - 1 {
        let current_symbol: char = expression.chars().nth(i).unwrap();
        if check_operation(current_symbol) {
            tokens.push((current_symbol).to_string().clone());
        } else if check_number(current_symbol) {
            add_number(expression.clone(), tokens, &mut i);
            continue;
        } else if current_symbol == 'x' {
            tokens.push((current_symbol).to_string().clone());
        } else {
            add_func(expression.clone(), tokens, &mut i);
            continue;
        }
        i += 1;
    }
}

fn check(tokens: Vec<String>) -> bool {
    for elem in tokens {
        if !check_str_number(elem.clone()) &&
            !check_operation(elem.chars().nth(0).unwrap()) &&
            !check_function(elem.clone()) {
            return false;
        }
    }

    true
}

fn add_number(expression: String, tokens: &mut Vec<String>, i : &mut usize) {
    let mut number : String = String::new();
    let mut symbol : char = expression.chars().nth(*i).unwrap();

    while check_number(symbol) {
        number.push(symbol);
        *i += 1;
        symbol = expression.chars().nth(*i).unwrap();
    }

    tokens.push(number.clone());
}

fn add_func(expression: String, tokens: &mut Vec<String>, i : &mut usize) {
    let mut func : String = String::new();
    let mut symbol : char = expression.chars().nth(*i).unwrap();
    while check_operation(symbol) == false {
        func.push(symbol);
        *i += 1;
        symbol = expression.chars().nth(*i).unwrap();
    }
    tokens.push(func.clone());
}
