use crate::common::*;

pub fn transfer_to_polish(tokens : Vec<String>, polish_tokens : &mut Vec<String>) {
    let mut stack = Vec::new();

    for elem in tokens {
        if check_str_number(elem.clone()) || elem.clone() == "x" {
            polish_tokens.push(elem.clone());
        } else if elem == "(" || check_function(elem.clone()) {
            stack.push(elem.clone());
        } else if elem == ")" {
            while stack.len() > 0 && stack.last().unwrap().clone() != "(".to_string() {
                polish_tokens.push(stack.pop().unwrap().clone())
            }
            stack.pop();
        } else if check_operation(elem.clone()) != -1 {
            let cur_priority_op = check_operation(elem.clone());

            while stack.len() > 0 && (check_operation(stack.last().unwrap().clone()) >= cur_priority_op) {
                polish_tokens.push(stack.pop().unwrap());
            }
            stack.push(elem.clone());
        }
    }

    while stack.len() > 0 {
        polish_tokens.push(stack.pop().unwrap());
    }
}
