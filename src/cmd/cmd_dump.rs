use crate::png::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(required = true, value_name = "INPUT_FILE", help = "The source file")]
    src: String,
}

pub fn run(args: &Args) {
    let png = PngFile::new(&args.src);
    match png {
        Ok(mut png) => png.dump(),
        Err(e) => {
            eprintln!("Error loading png file: {}", e.to_string());
        }
    }
}
