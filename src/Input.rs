use std::io;

pub fn input() -> bool {
    // input expration
    let mut expration = String::new();
    match get_expration(&mut expration) {
        true => {},
        false => {return false;}
    }

    println!("{}", expration);

    // return
    true
}

fn get_expration(expration : &mut String) -> bool {
    match io::stdin().read_line(expration) {
        Ok(_) => true,
        Err(e) => false
    }
}
