use std::any::Any;
use std::io;

pub fn input(lexems : &mut Vec<String>) -> bool {
    // input expration
    let mut expration : String = String::new();
    match get_expration(&mut expration) {
        true => {
            // parsing expration
            parsing(lexems, expration);

            return check(lexems.clone());
        },
        false => {return false;}
    }
}

fn get_expration(expration : &mut String) -> bool {
    match io::stdin().read_line(expration) {
        Ok(_) => true,
        Err(e) => false
    }
}

fn parsing(lexems : &mut Vec<String>, expration : String) {
    let mut i = 0;
    while i < expration.len() - 1 {
        let curent_symbol : char = expration.chars().nth(i).unwrap();
        if check_operation(curent_symbol) {
            lexems.push((curent_symbol).to_string().clone());
        } else if check_number(curent_symbol) {
            add_number(expration.clone(), lexems, &mut i);
            continue;
        } else if curent_symbol == 'x' {
            lexems.push((curent_symbol).to_string().clone());
        } else {
            add_func(expration.clone(), lexems, &mut i);
            continue;
        }
        i += 1;
    }
}

fn check(lexems : Vec<String>) -> bool {
    for elem in lexems {
        if !check_str_number(elem.clone()) &&
            !check_operation(elem.chars().nth(0).unwrap()) &&
            !check_function(elem.clone()) {
            return false;
        }
    }

    true
}

fn check_str_number(str : String) -> bool {
    for n in str.chars() {
        if !check_number(n) { return false; }
    }
    true
}

fn check_operation(symbol : char) -> bool {
    if symbol == '+' || symbol == '-' || symbol == '*' || symbol == '/' ||
        symbol == '(' || symbol == ')' {
        return true;
    }
    false
}

fn check_function(str : String) -> bool {
    if str.eq(&mut "sin".to_string()) ||
        str.eq(&mut "cos".to_string()) ||
        str.eq(&mut "tn".to_string()) ||
        str.eq(&mut "ctg".to_string()) ||
        str.eq(&mut "sqrt".to_string()) ||
        str.eq(&mut "x".to_string())
    { return true; }
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
    let mut number : String = String::new();
    let mut symbol : char = expration.chars().nth(*i).unwrap();
    while check_number(symbol) {
        number.push(symbol);
        *i += 1;
        symbol = expration.chars().nth(*i).unwrap();
    }
    lexems.push(number.clone());
}

fn add_func(expration: String, lexems : &mut Vec<String>, i : &mut usize) {
    let mut func : String = String::new();
    let mut symbol : char = expration.chars().nth(*i).unwrap();
    while check_operation(symbol) == false {
        func.push(symbol);
        *i += 1;
        symbol = expration.chars().nth(*i).unwrap();
    }
    lexems.push(func.clone());
}
