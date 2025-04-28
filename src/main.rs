use std::io;
fn main() {
    println!("Hello! Welcome to the Blockchain World");
    println!("Please enter the websocket data for the blockchain");

    let mut new = String::new();
    io::stdin()
        .read_line(&mut new)
        .expect("Failed to read line");

    let mut check: std::str::CharIndices<'_> = new.char_indices();
    if check.next() == Some((0, 'w')) && check.next() ==  Some((1,'s')) {
        println!("Checking the data");
    }
    else{
        println!("Please enter the correct endpoint");
    }

}