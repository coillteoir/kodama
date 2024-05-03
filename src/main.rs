struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}

fn main() {
    println!("{}", compile("cuboid 2 3 4".to_string()))
}

fn prism() -> String {
    "".to_string()
}

fn cuboid(sx: f32, sy: f32, sz: f32) -> String {
    println!("GENERATING CUBOID");
    if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
        return "could not generate cuboid, side length less than or equal to zero".to_string();
    }
    let points = [
        Point::new(0.0, sy, sz),
        Point::new(0.0, 0.0, sz),
        Point::new(sx, 0.0, sz),
        Point::new(sx, sy, sz),
        Point::new(0.0, sy, 0.0),
        Point::new(0.0, 0.0, 0.0),
        Point::new(sx, 0.0, 0.0),
        Point::new(sx, sy, 0.0),
    ];
    let vertexString = points
        .into_iter()
        .map(|p| format!("v {0} {1} {2}", p.x, p.y, p.z))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        r#"{0}
f -8 -7 -6 -5
f -1 -2 -3 -4
f -5 -6 -2 -1
f -4 -8 -5 -1
f -4 -3 -7 -8
f -7 -3 -2 -6
"#,
        vertexString
    )
}

fn cube(x: f32, y: f32, z: f32, size: f32) -> String {
    println!("GENERATING CUBE");
    if size <= 0.0 {
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
v {size_x} {y} {z}
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
            "cuboid" => result.push_str(&cuboid(
                tokens[1].parse::<f32>().expect("non numeric value given"),
                tokens[2].parse::<f32>().expect("non numeric value given"),
                tokens[3].parse::<f32>().expect("non numeric value given"),
            )),
            &_ => println!("{} not supported", tokens[0]),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use std::fs;
    #[test]
    fn test_compile_cube() {
        let input = fs::read_to_string("./tests/cube/main.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cube/result.obj").expect("could not load file");
        assert_eq!(compile(input), result);
    }
    #[test]
    fn test_compile_cuboid() {
        let input = fs::read_to_string("./tests/cuboid/main.kda").expect("could not load file");
        let result = fs::read_to_string("./tests/cuboid/result.obj").expect("could not load file");
        assert_eq!(compile(input), result);
    }
}
