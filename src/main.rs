mod cli;
mod backend;
mod cellar;


/// is this thing on?
fn main() {
    // Move this to a function in cli/mod.rs and call it from here. This is the main entry point for the CLI.?
    cli::Cli::main();
}
