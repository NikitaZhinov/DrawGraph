use std::arch::x86_64::_tzcnt_u32;
use std::io;
use std::ops::Add;
use std::str::Chars;
use std::sync::mpsc::sync_channel;

pub fn input(lexems : &mut Vec<String>) -> bool {
    // input expration
    let mut expration = String::new();
    match get_expration(&mut expration) {
        true => {},
        false => {return false;}
    }

    // parsing expration
    parsing(lexems, expration);

    true
}

fn get_expration(expration : &mut String) -> bool {
    match io::stdin().read_line(expration) {
        Ok(_) => true,
        Err(e) => false
    }
}

fn parsing(lexems : &mut Vec<String>, expration : String) {
    let mut i = 0;
    while i < expration.len() {
        let curent_symbol = expration.chars().nth(i).unwrap();
        if check_operation(curent_symbol) {
            lexems.push((curent_symbol).to_string().clone());
        } else if check_number(curent_symbol) {
            add_number(expration.clone(), lexems, &mut i);
            continue;
        }
        i += 1;
    }
}

fn check_operation(symbol : char) -> bool {
    if symbol == '+' || symbol == '-' || symbol == '*' || symbol == '/' ||
        symbol == '(' || symbol == ')' {
        return true;
    }
    false
}

fn check_number(symbol : char) -> bool {
    if symbol.to_ascii_lowercase() >= '0'.to_ascii_lowercase() &&
        symbol.to_ascii_lowercase() <= '9'.to_ascii_lowercase() {
        return true;
    }
    false
}

fn add_number(expration: String, lexems : &mut Vec<String>, i : &mut usize) {
    let mut number = String::new();
    let mut symbol = expration.chars().nth(*i).unwrap();
    while check_number(symbol) {
        number.push(symbol);
        *i += 1;
        symbol = expration.chars().nth(*i).unwrap();
    }
    lexems.push(number.clone());
}
