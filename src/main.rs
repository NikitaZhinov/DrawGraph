mod common;
mod input;
mod polish_notation;
mod draw;

fn main() {
    let mut tokens = Vec::new();

    match input::get_expression(&mut tokens) {
        true => {
            let mut polish_tokens = Vec::new();
            polish_notation::transfer_to_polish(tokens.clone(), &mut polish_tokens);
            // draw::draw_map(polish_tokens);
            println!("{:?}", tokens);
            println!("{:?}", polish_tokens);
        },
        false => println!("Error: failed or uncorrected input!")
    }
}
