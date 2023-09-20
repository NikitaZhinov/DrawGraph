mod common;
mod input;
mod polish_notation;
mod draw;

fn main() {
    let mut tokens = Vec::new();

    match input::get_expression(&mut tokens) {
        0 => {
            // println!("{:?}", tokens);
            let mut polish_tokens = Vec::new();
            polish_notation::transfer_to_polish(tokens.clone(), &mut polish_tokens);
            // println!("{:?}", polish_tokens);
            draw::draw_map(polish_tokens.clone());
        },
        1 => println!("Error: failed input!"),
        2 => println!("Error: uncorrected input!"),
        _ => println!("Error!")
    }
}
