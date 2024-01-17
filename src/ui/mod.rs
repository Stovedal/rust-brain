use std::io::{self};

pub fn write_introduction() {
    println!("Welcome to Sellpy brain, ask a question! \n\n");
}

pub fn read_question() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Something is terribly wrong with the IO.");

    let clean_user_input = user_input.replace('\n', "").into();

    return clean_user_input;
}

pub fn write_response(response: &str) {
    println!("\nAnswer: {}\n", response);
}

pub fn write_error() {
    println!("Something went wrong, please try again.");
}
