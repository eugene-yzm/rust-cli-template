use clap::Parser;
use rust_cli_template::cli::{cli_args, cli_commands};

fn main() {
    let cli = cli_args::CliArgs::parse();

    println!("Value for arg1 = {}", cli.arg1);
    
    if let Some(arg2) = cli.arg2 {
        println!("Value for arg2: {arg2}");
    }

    match &cli.cmd1 {
        cli_commands::CliCommand::Add(name) => {
            println!("'Add' was used, name is: {:?}", name.name)
        }
    }
}
