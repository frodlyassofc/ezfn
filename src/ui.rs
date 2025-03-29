use std::io;

pub struct UserInterface;

impl UserInterface {
    pub fn display_message(message: &str) {
        println!("{}", message);
    }

    pub fn get_input(prompt: &str) -> String {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}