mod meta_command;

use std::io::Write;

fn main() {
    loop {
        show_prompt();

        let buffer = read_input();

        if buffer == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'.", buffer);
        }
    }
}

fn show_prompt() {
    print!("db > ");
    std::io::stdout().flush().expect("Cannot flush stdout");
}

fn read_input() -> String {
    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input!");

    buffer.trim().into()
}
