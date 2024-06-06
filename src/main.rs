use kodama::compiler::compile;
use clap::{Parser, Subcommand};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about="3d modelling but epic", long_about="lengthy epic")]
struct Args {
    #[arg(help="source file")]
    source: String,
    #[arg(help="output file")]
    output: String
}

fn main() {

    let args = Args::parse();

    let source_path = Path::new(&args.source);
    let mut output_path = Path::new(&args.output);

    if source_path.extension().and_then(OsStr::to_str) != Some("kda") {
        eprintln!("please specify a kodama source file");
        return;
    }
    let source = fs::read_to_string(source_path).expect("couldn't load source file");

    match output_path.extension().and_then(OsStr::to_str) {
        Some("obj") => println!("{}", compile(&source)),
        Some(e) => eprintln!("file type {} not supported", e),
        None => eprintln!("please specify an output file type"),
    }
}
