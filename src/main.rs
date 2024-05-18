use kodama::compile;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_path = Path::new(&args[1]);
    let mut output_path = Path::new("");
    if args.len() == 3 {
        output_path = Path::new(&args[2]);
    }
    if source_path.extension().and_then(OsStr::to_str) != Some("kda") {
        eprintln!("please specify a kodama source file");
        return;
    }
    let source = fs::read_to_string(&source_path).expect("couldn't load source file");

    match output_path.extension().and_then(OsStr::to_str) {
        Some("obj") => println!("{}", compile(&source)),
        Some(e) => eprintln!("file type {} not supported", e),
        None => eprintln!("please specify an output file type"),
    }
}
