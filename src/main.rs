mod Input;
// mod PolishNotation;
// mod Stack;

fn main() {
    let mut lexems = Vec::new();

    match Input::input(&mut lexems) {
        true => {
            // let mut polish_lexems : Vec<String> = vec!();
            // match PolishNotation::polish_notation(lexems, &mut polish_lexems) {
            //     true => {
            //
            //     },
            //     false => println!("Error: uncorrected input!")
            // }

            println!("{:?}", lexems);
        },
        false => println!("Error: failed or uncorrected input!")
    }
}
