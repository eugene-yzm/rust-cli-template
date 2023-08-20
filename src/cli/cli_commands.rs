use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    /// List all available commands
    List, 

    /// Cmd with its own set of arg configs
    Add { name: Option<String> },
}
