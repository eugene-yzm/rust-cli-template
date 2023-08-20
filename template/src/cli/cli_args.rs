use clap::Parser;
use super::cli_commands::CliCommand;

// Command line arguments that this program is expected to parse
// Derive reference: https://docs.rs/clap/latest/clap/_derive/index.html
// Arg Struct: https://docs.rs/clap/latest/clap/struct.Arg.html
// Derive arg attribtures: https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct CliArgs {
    /// Description for command1
    #[command(subcommand)]
    pub cmd1: CliCommand,
    
    /// Required arg: unsigned 8-bit value
    #[arg(short = 'S', long, value_name = "U8_VALUE")]
    #[arg(default_value_t = 42)]
    pub arg1: u8,

    /// Optional arg: string value
    #[arg(short = 'T', long, value_name = "STR_VALUE")]
    pub arg2: Option<String>
}
