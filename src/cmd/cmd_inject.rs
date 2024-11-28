use crate::png::*;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(required = true, help = "The source png file")]
    src: String,

    #[arg(
        required = true,
        help = "The destination png file where to inject the new chunk"
    )]
    dst: String,
}

pub fn run(args: &Args) {
    let png = PngFile::new(&args.src);
    match png {
        Ok(mut png) => _ = png.inject(&args.dst),
        Err(e) => {
            eprintln!("Error loading png file: {}", e.to_string());
        }
    }
}
