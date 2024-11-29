use crate::png::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        required = true,
        value_name = "INPUT_FILE",
        help = "The source file where the hidden data is stored"
    )]
    src: String,
}

pub fn run(args: &Args) {
    let mut png = match PngFile::new(&args.src) {
        Ok(png) => png,
        Err(e) => {
            eprintln!("Error loading png file: {}", e);
            return;
        }
    };

    png.extract().unwrap_or_else(|e| eprintln!("{}", e));
}
