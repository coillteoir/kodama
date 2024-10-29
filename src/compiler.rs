use crate::shape::{cone, cube, cuboid, sphere, Point};

pub fn compile(data: &str) -> Result<String, String> {
    let mut result = String::new();
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
    Ok(result)
}
