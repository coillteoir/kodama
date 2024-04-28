
fn main() {
    println!("{}", compile("cube 1".to_string()))
}

fn cube(size: i32) -> String {
format!(r#"v 0.000000 {size}.000000 {size}.000000
v 0.000000 0.000000 {size}.000000
v {size}.000000 0.000000 {size}.000000
v {size}.000000 {size}.000000 {size}.000000
v 0.000000 {size}.000000 0.000000
v 0.000000 0.000000 0.000000
v {size}.000000 0.000000 0.000000
v {size}.000000 {size}.000000 0.000000
f 1 2 3 4
f 8 7 6 5
f 4 3 7 8
f 5 1 4 8
f 5 6 2 1
f 2 6 7 3
"#)
}

fn compile(data: String) -> String {

    let mut result = String::from("");

    let lines = data.split("\n");
    for line in lines {
       let tokens: Vec<&str> = line.split(" ").collect();
       if tokens[0] == "cube"{
           result.push_str(
               &cube(tokens[1].parse::<i32>().expect("invalid value given"))
            )
       }
    }
    return result
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::compile;
    #[test]
    fn test_compile(){
        let input = fs::read_to_string("./tests/cube/cube.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cube/cube.obj").expect("could not load file");
        assert_eq!(compile(input), result)
    }
}
