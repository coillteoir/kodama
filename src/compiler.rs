use crate::shape::{cone, cube, cuboid, sphere, Point};

#[derive(Debug, Clone)]
struct Token {}

fn validate_braces(source: &str) -> Result<(), String> {
    let braces = String::from(source)
        .chars()
        .filter(|c| (*c == '{' || *c == '}'))
        .collect::<Vec<char>>();

    if braces.len() % 2 != 0 {
        return Err(String::from("invalid amount of braces"));
    }
    let mut stack = Vec::<char>::new();
    for brace in braces.into_iter() {
        if brace == '{' {
            stack.push(brace);
        }

        if brace == '}' {
            stack.pop();
        }
    }
    if !stack.is_empty() {
        return Err(String::from("mismatched braces"));
    }
    Ok(())
}

fn parser(data: &str) -> Result<String, String> {
    let _ = validate_braces(data);
    Ok(String::from("yay"))
}

pub fn compile(data: &str) -> String {
    let mut result = String::new();
    let _ = parser(data);
    let lines = data.split('\n');
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        match *tokens.first().unwrap() {
            "cube" => result.push_str(
                &cube(
                    Point::new(0.0, 0.0, 0.0),
                    tokens[1].parse::<f32>().expect("invalid value given"),
                )
                .unwrap(),
            ),
            "cuboid" => result.push_str(
                &cuboid(
                    Point::new(0.0, 0.0, 0.0),
                    tokens[1].parse::<f32>().expect("non numeric value given"),
                    tokens[2].parse::<f32>().expect("non numeric value given"),
                    tokens[3].parse::<f32>().expect("non numeric value given"),
                )
                .unwrap(),
            ),
            "cone" => result.push_str(&cone(
                Point::new(0.0, 0.0, 0.0),
                0,
                tokens[1].parse::<f32>().expect("non numeric value given"),
            )),
            "sphere" => result.push_str(
                &sphere(
                    Point::new(0.0, 0.0, 0.0),
                    tokens[1].parse::<f32>().expect("non numeric value given"),
                    10,
                )
                .unwrap(),
            ),
            "#" => todo!(),
            &_ => eprintln!("{} not supported", tokens[0]),
        }
    }
    result
}
