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
    Dump(cmd_dump::Args),
    Inject(cmd_inject::Args),
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
