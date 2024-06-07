use clap::Parser;
use kodama::compiler::compile;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(
    author = "David Lynch",
    about = "3d modelling but epic",
    long_about = "lengthy epic"
)]
struct Args {
    #[arg(help = "source file")]
    source: String,
    #[arg(help = "output file", default_value="")]
    output: String,
}

fn main() {

    println!("{}", compile("sphere 1"));

    let args = Args::parse();

    let source_path = Path::new(&args.source);
    let output_path = Path::new(&args.output);

    if source_path.extension().and_then(OsStr::to_str) != Some("kda") {
        eprintln!("please specify a kodama source file");
        return;
    }
    let source = fs::read_to_string(source_path).expect("couldn't load source file");

    match output_path.extension().and_then(OsStr::to_str) {
        Some("obj") => println!("{}", compile(&source)),
        Some(other) => eprintln!("file type {other} not supported"),
        None => println!("{}", compile(&source)),
    }
}
