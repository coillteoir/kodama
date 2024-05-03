use std::f32::consts::PI;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

fn main() {
    println!("{}", compile("prism 1".to_string()).unwrap())
}

fn vertex_string(points: Vec<Point>) -> String {
    points
        .into_iter()
        .map(|p| format!("v {0} {1} {2}", p.x, p.y, p.z))
        .collect::<Vec<String>>()
        .join("\n")
}

fn prism(_radius: f32) -> Result<String, String> {
    let points = [
        Point::new(0.000000, (PI / 3.0).sin(), 0.000000),
        Point::new(0.0, 0.000000, 0.0_f32.cos()),
        Point::new((4.0 * PI / 3.0).sin(), 0.000000, (2.0 * -PI / 3.0).cos()),
        Point::new((2.0 * PI / 3.0).sin(), 0.000000, (2.0 * -PI / 3.0).cos()),
    ];

    Ok(format!(
        r#"{0}
f -3 -2 -1
f -4 -2 -1
f -4 -3 -1
f -4 -2 -3
"#,
        vertex_string(points.to_vec())
    ))
}

fn cuboid(sx: f32, sy: f32, sz: f32) -> Result<String, String> {
    println!("GENERATING CUBOID");
    if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
        return Err(
            "could not generate cuboid, side length less than or equal to zero".to_string(),
        );
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
    Ok(format!(
        r#"{0}
f -8 -7 -6 -5
f -1 -2 -3 -4
f -5 -6 -2 -1
f -4 -8 -5 -1
f -4 -3 -7 -8
f -7 -3 -2 -6
"#,
        vertex_string(points.to_vec())
    ))
}

fn cube(size: f32) -> Result<String, String> {
    println!("GENERATING CUBE");
    if size <= 0.0 {
        return Err("ERROR: cannot generate cube of size less than zero".to_string());
    }
    cuboid(size, size, size)
}

fn compile(data: String) -> Result<String, String> {
    let mut result = String::from("");

    let lines = data.split('\n');
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0] {
            "cube" => result
                .push_str(&cube(tokens[1].parse::<f32>().expect("invalid value given")).unwrap()),
            "cuboid" => result.push_str(
                &cuboid(
                    tokens[1].parse::<f32>().expect("non numeric value given"),
                    tokens[2].parse::<f32>().expect("non numeric value given"),
                    tokens[3].parse::<f32>().expect("non numeric value given"),
                )
                .unwrap(),
            ),
            "prism" => result.push_str(
                &prism(tokens[1].parse::<f32>().expect("non numeric value given")).unwrap(),
            ),
            &_ => println!("{} not supported", tokens[0]),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use std::fs;

    fn run_test(input_path: String, result_path: String) {
        let input = fs::read_to_string(&input_path).expect("could not load file");
        let result = fs::read_to_string(&result_path).expect("could not load file");
        assert_eq!(compile(input), Ok(result));
    }
    #[test]
    fn walk_tests() {
        match fs::read_dir("./tests/") {
            Ok(entries) => {
                for entry in entries {
                    let path = entry.expect("could not open dir").path();
                    let input = path.join("main.kda");
                    let output = path.join("result.obj");
                    run_test(
                        input.into_os_string().into_string().unwrap(),
                        output.into_os_string().into_string().unwrap(),
                    )
                }
            }
            Err(e) => eprintln!("{}", e)
        }
    }
}
