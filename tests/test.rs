#[cfg(test)]
mod tests {
    use kodama::compile;
    use std::fs;

    fn run_test(input_path: String, result_path: String) {
        let input = fs::read_to_string(&input_path).expect("could not load file");
        let result = fs::read_to_string(&result_path).expect("could not load file");
        assert_eq!(compile(&input), result);
    }
    #[test]
    fn walk_tests() {
        let test_dir = "./tests";
        match fs::read_dir(test_dir) {
            Ok(entries) => {
                for entry in entries {
                    let path = entry.expect("could not open dir").path();
                    if !path.is_dir() {
                        continue;
                    }
                    let input = path.join("main.kda");
                    let output = path.join("result.obj");
                    run_test(
                        input.into_os_string().into_string().unwrap(),
                        output.into_os_string().into_string().unwrap(),
                    )
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
