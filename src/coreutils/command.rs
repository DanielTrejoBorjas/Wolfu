
// base template for commands in the coreutils module
// This trait defines the basic structure for commands in the coreutils module.
pub trait Command {
    // Returns the name of the command
    // This is used to register the command and identify it in the command map.
    fn name(&self) -> &'static str;

    // Runs the command with the provided arguments
    // Returns Ok(()) on success or an error message on failure.
    fn run(&self, args: &[String]) -> Result<(), String>;

    // Returns a help message for the command
    // This is used to provide usage information when the command is invoked with a help flag or when the command is unknown.
    // If no help is available, it can return a default message.
    fn help(&self) -> &'static str {
        "No help available for this command."
    }
}
