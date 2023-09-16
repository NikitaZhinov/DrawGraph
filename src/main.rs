mod common;
mod input;
mod polish_notation;

fn main() {
    let mut tokens = Vec::new();

    match input::get_expression(&mut tokens) {
        true => {
            let mut polish_tokens : Vec<String> = vec!();
            match polish_notation::transfer_to_polish(tokens, &mut polish_tokens) {
                true => {
                    println!("{:?}", polish_tokens);
                },
                false => println!("Error: uncorrected input!")
            }

            // println!("{:?}", tokens);
        },
        false => println!("Error: failed or uncorrected input!")
    }
}
