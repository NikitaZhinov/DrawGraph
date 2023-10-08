use std::io;
use crate::common::*;

pub fn get_expression(tokens: &mut Vec<String>) -> u8 {
    // input expression
    let mut expression = String::new();
    return match io::stdin().read_line(&mut expression) {
        Ok(_) => {
            // parsing expression
            parsing(tokens, expression);

            check(tokens)
        },
        Err(_) => { 1 }
    }
}

fn parsing(tokens: &mut Vec<String>, expression: String) {
    let mut i = 0;
    while i < expression.len() - 1 {
        let current_symbol = expression.chars().nth(i).unwrap();
        if check_operation(current_symbol.to_string()) != -1 {
            tokens.push((current_symbol).to_string().clone());
        } else if check_number(current_symbol) {
            add_number(expression.clone(), tokens, &mut i);
            continue;
        } else if current_symbol == 'x' {
            tokens.push(current_symbol.to_string().clone());
        } else if current_symbol.is_alphabetic() {
            add_func(expression.clone(), tokens, &mut i);
            continue;
        }
        i += 1;
    }
}

fn check(tokens: &mut Vec<String>) -> u8 {
    // checking for errors
    for token in tokens.clone() {
        if !check_str_number(token.clone()) &&
            !(check_operation(token.clone()) != -1) &&
            !check_function(token.clone()) &&
            token != "x" {
            return 2;
        }
    }

    // find unary minus
    for i in 0..(tokens.len() - 1) {
        if tokens[i] == "-" {
            if i == 0 || (i > 0 && tokens[i - 1] == "(") {
                tokens[i] = "~".to_string();
            }
        } else if tokens[i] == "~" {
            if i != 0 && tokens[i - 1] == "~" {
                return 2;
            }
        }
    }

    0
}

fn add_number(expression: String, tokens: &mut Vec<String>, i : &mut usize) {
    let mut number = String::new();
    let mut symbol = expression.chars().nth(*i).unwrap();

    while check_number(symbol) {
        number.push(symbol);
        *i += 1;
        symbol = expression.chars().nth(*i).unwrap();
    }

    tokens.push(number.clone());
}

fn add_func(expression: String, tokens: &mut Vec<String>, i : &mut usize) {
    let mut func = String::new();
    let mut symbol = expression.chars().nth(*i).unwrap();
    while check_operation(symbol.to_string()) == -1 {
        func.push(symbol);
        *i += 1;
        symbol = expression.chars().nth(*i).unwrap();
    }
    tokens.push(func.clone());
}
