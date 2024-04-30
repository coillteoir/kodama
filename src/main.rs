fn main() {
    println!("{}", compile("cuboid 1 4 2".to_string()).unwrap());
}

fn cuboid(sx: f32, sy: f32, sz: f32) -> Result<String, String> {
    println!("GENERATING CUBOID");
    if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
        return Err(
            "could not generate cuboid, side length less than or equal to zero".to_string(),
        );
    }
    Ok(format!(
        r#"v 0 {sy} {sz}
v 0 0 {sz}
v {sx} 0 {sz}
v {sx} {sy} {sz}
v 0 {sy} 0
v 0 0 0
v {sx} 0 0
v {sx} {sy} 0
f -8 -7 -6 -5
f -1 -2 -3 -4
f -5 -6 -2 -1
f -4 -8 -5 -1
f -4 -3 -7 -8
f -7 -3 -2 -6
"#
    ))
}

fn cube(_x: f32, _y: f32, _z: f32, size: f32) -> Result<String, String> {
    println!("GENERATING CUBE");
    if size <= 0.0 {
        return Err(String::from(
            "ERROR: cannot generate cube of size less than zero",
        ));
    }
    cuboid(size, size, size)
}

fn compile(data: String) -> Result<String, String> {
    let mut result = String::new();
    let mut error = String::new();

    let lines = data.split('\n');
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        let mut data = String::new();
        match tokens.first() {
            Some(&"cube") => match cube(
                0.0,
                0.0,
                0.0,
                tokens
                    .get(1)
                    .expect("please specify a size argument")
                    .parse::<f32>()
                    .expect("invalid value given"),
            ) {
                Ok(g) => data = g,
                Err(e) => error.push_str(&e),
            },
            Some(&"cuboid") => match cuboid(
                tokens
                    .get(1)
                    .expect("invalid number of arguments in cuboid")
                    .parse::<f32>()
                    .expect("non numeric value given"),
                tokens
                    .get(2)
                    .expect("invalid number of arguments in cuboid")
                    .parse::<f32>()
                    .expect("non numeric value given"),
                tokens
                    .get(3)
                    .expect("invalid number of arguments in cuboid")
                    .parse::<f32>()
                    .expect("non numeric value given"),
            ) {
                Ok(g) => data = g,
                Err(e) => error.push_str(&e),
            },
            Some(&_) => eprintln!("{} not supported", tokens[0]),
            None => unimplemented!(),
        }
        result.push_str(&data);
    }
    if !error.is_empty() {
        return Err(error);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use std::fs;
    #[test]
    fn test_compile_cube() {
        let input = fs::read_to_string("./tests/cube/main.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cube/result.obj").expect("could not load file");
        assert_eq!(compile(input), Ok(result));
    }
    #[test]
    fn test_compile_cuboid() {
        let input = fs::read_to_string("./tests/cuboid/main.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cuboid/result.obj").expect("could not load file");
        assert_eq!(compile(input), Ok(result));
    }
}
