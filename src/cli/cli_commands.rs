use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct AddCmdArgs {
    pub name: String
} 

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    /// Cmd with its own set of arg configs
    Add(AddCmdArgs),
}
