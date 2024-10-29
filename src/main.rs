use clap::Parser;
use kodama::compiler::*;
use std::fs;

#[derive(Parser)]
#[command(author = "David Lynch", about = "3d modelling but epic")]
struct Args {
    #[arg(short, long, help = "source file")]
    source: String,
    #[arg(short, long, help = "output file")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.source).expect("File not found");
    if let Ok(result) = compile(&contents) {
        if let Err(e) = fs::write(args.output, result) {
            panic!("{}", e)
        }
    }
}
