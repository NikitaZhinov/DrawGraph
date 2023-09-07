mod Input;

fn main() {

    match Input::input() {
        true => {
            println!("nice");
        },
        false => println!("Error: failed input!")
    }
}


