mod Input;

fn main() {
    let mut lexems = Vec::new();

    match Input::input(&mut lexems) {
        true => {
            println!("{:?}", &lexems);
        },
        false => println!("Error: failed input!")
    }
}
