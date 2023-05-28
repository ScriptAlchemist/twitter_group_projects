use std::io;
#[allow(unused)]

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn main() {
    let clear_screen = "\x1b[2J";
    let set_cursor = "\x1b[1;1H";
    let red = "\x1b[31m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let clear_color = "\x1b[0m";

    print!("{}{}", clear_screen, set_cursor);

    println!(
        "{}Welcome{} to the {}Guessing{} {}Game!{}\n",
        red, clear_color, green, clear_color, yellow, clear_color
    );
    println!("What is your name?\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading name");
    let input = input.trim();
    println!("Hello {}, let us play a game!\n", input);

    let mut input = String::new();
    println!("Would you like to play: Easy, Medium, or Hard?\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading name");
}
