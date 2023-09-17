use thiserror::Error;

#[derive(Debug, Error)]
#[error("Unrecognized command '{0}'.")]
pub struct UnrecognizedCommand(String);

pub fn do_meta_command(command: &String) -> Result<(), UnrecognizedCommand> {
    match command.as_str() {
        ".exit" => std::process::exit(0),
        _ => Err(UnrecognizedCommand(command.to_owned())),
    }
}
