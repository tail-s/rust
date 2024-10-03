// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {

    first_name();
    last_name();

    let first = "Sojin";
    let last = "Kim";
    println!("{:?} {:?}", first, last);
    println!("{} {}", first, last);
}

fn first_name() {
    println!("Sojin");
}

fn last_name() {
    println!("Kim");
}