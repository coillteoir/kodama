fn main() {
    println!("{}", compile("cube 1".to_string()))
}

fn cube(x: f32, y: f32, z: f32, size: f32) -> String {
    if size < 0.0 {
        return "ERROR: cannot generate cube of size less than zero".to_string();
    }
    let size_x = size + x;
    let size_y = size + y;
    let size_z = size + z;
    format!(
        r#"v {x} {size_y} {size_z}
v {x} {y} {size_z}
v {size_x} {y} {size_z}
v {size_x} {size_y} {size_z}
v {x} {size_y} {z}
v {x} {y} {z}
v {size} {y} {z}
v {size_x} {size_y} {z}
f -8 -7 -6 -5
f -1 -2 -3 -4
f -5 -6 -2 -1
f -4 -8 -5 -1
f -4 -3 -7 -8
f -7 -3 -2 -6
"#
    )
}

fn compile(data: String) -> String {
    let mut result = String::from("");

    let lines = data.split('\n');
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0] {
            "cube" => result.push_str(&cube(
                0.0,
                0.0,
                0.0,
                tokens[1].parse::<f32>().expect("invalid value given"),
            )),
            &_ => println!("not supported yet"),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use std::fs;
    #[test]
    fn test_compile() {
        let input = fs::read_to_string("./tests/cube/cube.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cube/cube.obj").expect("could not load file");
        assert_eq!(compile(input), result)
    }
}
