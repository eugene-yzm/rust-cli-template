use clap::Parser;
use super::cli_commands::CliCommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct CliArgs {
    // Command line arguments that this program is expected to parse
    // Derive reference: https://docs.rs/clap/latest/clap/_derive/index.html
    // Arg Struct: https://docs.rs/clap/latest/clap/struct.Arg.html
    // Derive arg attribtures: https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes

    /// Description for command1
    #[command(subcommand)]
    cmd1: CliCommand,
    
    /// Description for arg1
    #[arg(short = 'S', long, value_name = "ARG_VALUE")]
    arg1: Option<String>
}
