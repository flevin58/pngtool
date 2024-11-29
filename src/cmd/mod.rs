mod cmd_dump;
mod cmd_extract;
mod cmd_inject;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct CmdArgs {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Dumps the header and chunks (without data) to stdout")]
    Dump(cmd_dump::Args),
    #[command(about = "Injects a hidden message")]
    Inject(cmd_inject::Args),
    #[command(about = "Extracts the hidden messa if present")]
    Extract(cmd_extract::Args),
}

pub fn parse_and_run() {
    let args = CmdArgs::parse();
    //dbg!(&args);

    match &args.command {
        Some(Commands::Dump(args)) => cmd_dump::run(&args),
        Some(Commands::Inject(args)) => cmd_inject::run(&args),
        Some(Commands::Extract(args)) => cmd_extract::run(&args),
        None => {
            println!("No subcommand given");
        }
    }
}
