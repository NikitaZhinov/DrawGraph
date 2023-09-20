use crate::common::*;

static X_ : f64 = 4.0 * std::f64::consts::PI / 80.0;
static Y_ : f64 = 2.0 / 25.0;

pub fn draw_map(polish_tokens : Vec<String>) {
    let mut ym = 1.0 - Y_;
    while ym > -1.0 {
        for xm in 0..80 {
            let x = (xm as f64) * X_;
            let y = count(polish_tokens.clone(), x);
            if y < ym + Y_ && y >= ym {
                print!("*");
            } else {
                if x == 0.0 {
                    print!("|");
                } else if ym < Y_ / 2.0 && ym > -Y_ / 2.0 {
                    print!("-");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
        ym -= Y_;
    }
}

fn count(polish_tokens : Vec<String>, x : f64) -> f64 {
    let mut stack : Vec<f64> = Vec::new();

    for token in polish_tokens.clone() {
        if check_str_number(token.clone()) {
            stack.push(str_to_num(token.to_string()));
        } else if check_operation(token.clone()) != -1 {
            let mut res = 0.0;

            if token == "~".to_string() {
                let num = stack.pop().unwrap();
                res = 0.0 - num;
            } else {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();

                res = match token.chars().nth(0).unwrap() {
                    '+' => num1 + num2,
                    '-' => num1 - num2,
                    '*' => num1 * num2,
                    '/' => num1 / num2,
                    '^' => num1.powf(num2),
                    _ => 0.0
                };
            }

            stack.push(res);
        } else if check_function(token.clone()) {
            let num = stack.pop().unwrap();

            let res = match token.as_str() {
                "sin" => num.sin(),
                "cos" => num.cos(),
                "tg" => num.tan(),
                "ctg" => 1.0 / num.tan(),
                "sqrt" => num.sqrt(),
                "lg" => num.log10(),
                "ln" => num.ln(),
                _ => 0.0
            };

            stack.push(res);
        } else if token.clone() == "x" {
            stack.push(x);
        }
    }

    stack.pop().unwrap()
}

fn str_to_num(str_number : String) -> f64 {
    let mut number : f64 = 0.0;

    let mut i = 1;
    for n in str_number.chars() {
        let n_ : f64 = match n {
            '0' => 0.0,
            '1' => 1.0,
            '2' => 2.0,
            '3' => 3.0,
            '4' => 4.0,
            '5' => 5.0,
            '6' => 6.0,
            '7' => 7.0,
            '8' => 8.0,
            _ => 9.0
        };
        number += n_ * 10_f64.powf((str_number.len() - i) as f64);
        i += 1;
    }

    number
}
