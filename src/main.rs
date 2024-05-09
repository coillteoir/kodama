use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn to_obj_string(self) -> String {
        format!("v {0} {1} {2}", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
struct Face {
    points: Vec<Point>,
    indexes: Vec<u32>,
}

impl Face {
    pub fn new(points: Vec<Point>, indexes: Vec<u32>) -> Self {
        Self { points, indexes }
    }
    pub fn to_obj_string(&self) -> String {
        let mut result = "f ".to_string();

        result.push_str(
            &self
                .indexes
                .clone()
                .into_iter()
                .map(|i: u32| -> String {
                    format!("-{0}", (self.points.len() as u64 - u64::from(i)))
                })
                .collect::<Vec<String>>()
                .join(" "),
        );
        result
    }
}

fn main() {
    println!("{}", compile("sphere"));
}

fn vertex_string(points: Vec<Point>) -> String {
    points
        .into_iter()
        .map(Point::to_obj_string)
        .collect::<Vec<String>>()
        .join("\n")
}

fn sphere(origin: Point, radius: f32, detail: u32) -> Result<String, String> {
    if radius <= 0.0 {
        return Err("radius less than zero".to_string());
    }
    // Initialize top and bottom points
    let mut points = [
        Point::new(
            origin.x + (radius * (PI / 2.0).cos()),
            origin.y + (radius * (PI / 2.0).sin()),
            origin.z,
        ),
        Point::new(
            origin.x + (radius * (PI / -2.0).cos()),
            origin.y + (radius * (PI / -2.0).sin()),
            origin.z,
        ),
    ]
    .to_vec();

    for col in 0..detail {
        for row in 0..detail {
            points.push(Point::new(
                origin.x + (radius * ((2.0 * PI) / (detail as f32) * (col as f32)).cos()),
                origin.y + (radius * ((2.0 * PI) / (detail as f32) * (col as f32)).sin()),
                origin.z + (radius * ((2.0 * PI) / (detail as f32) * (row as f32)).sin()),
            ))
        }
    }
    println!("{:#?}", points);
    Ok(String::new())
}

fn cone(origin: Point, _detail: i32, _radius: f32) -> String {
    let points = [
        Point::new(origin.x, origin.y + (PI / 3.0).sin(), origin.z),
        Point::new(origin.x, origin.y, origin.z + 0.0_f32.cos()),
        Point::new(
            origin.x + (4.0 * PI / 3.0).sin(),
            origin.y,
            (2.0 * -PI / 3.0).cos(),
        ),
        Point::new(
            origin.x + (2.0 * PI / 3.0).sin(),
            origin.y,
            (2.0 * -PI / 3.0).cos(),
        ),
    ];

    let faces = [
        Face::new(points.to_vec(), vec![1, 2, 3]),
        Face::new(points.to_vec(), vec![0, 2, 3]),
        Face::new(points.to_vec(), vec![0, 1, 3]),
        Face::new(points.to_vec(), vec![0, 2, 1]),
    ];

    format!(
        r#"{0}
{1}
"#,
        vertex_string(points.to_vec()),
        faces
            .into_iter()
            .map(|f| f.to_obj_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn cuboid(origin: Point, sx: f32, sy: f32, sz: f32) -> Result<String, String> {
    println!("GENERATING CUBOID");
    if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
        return Err(
            "could not generate cuboid, side length less than or equal to zero".to_string(),
        );
    }
    let points = [
        Point::new(origin.x + 0.0, origin.y + sy, origin.z + sz),
        Point::new(origin.x + 0.0, origin.y + 0.0, origin.z + sz),
        Point::new(origin.x + sx, origin.y + 0.0, origin.z + sz),
        Point::new(origin.x + sx, origin.y + sy, origin.z + sz),
        Point::new(origin.x + 0.0, origin.y + sy, origin.z + 0.0),
        Point::new(origin.x + 0.0, origin.y + 0.0, origin.z + 0.0),
        Point::new(origin.x + sx, origin.y + 0.0, origin.z + 0.0),
        Point::new(origin.x + sx, origin.y + sy, origin.z + 0.0),
    ];

    let faces = [
        Face::new(points.to_vec(), vec![0, 1, 2, 3]),
        Face::new(points.to_vec(), vec![7, 6, 5, 4]),
        Face::new(points.to_vec(), vec![4, 5, 1, 0]),
        Face::new(points.to_vec(), vec![3, 7, 4, 0]),
        Face::new(points.to_vec(), vec![3, 2, 6, 7]),
        Face::new(points.to_vec(), vec![6, 2, 1, 5]),
    ];

    Ok(format!(
        r#"{0}
{1}
"#,
        vertex_string(points.to_vec()),
        faces
            .into_iter()
            .map(|f| f.to_obj_string())
            .collect::<Vec<String>>()
            .join("\n")
    ))
}

fn cube(origin: Point, size: f32) -> Result<String, String> {
    println!("GENERATING CUBE");
    if size <= 0.0 {
        return Err("ERROR: cannot generate cube of size less than zero".to_string());
    }
    cuboid(origin, size, size, size)
}

fn compile(data: &str) -> std::string::String {
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
            "sphere" => result.push_str(&sphere(Point::new(0.0, 0.0, 0.0), 10.0, 10).unwrap()),
            &_ => println!("{} not supported", tokens[0]),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::compile;
    use std::fs;

    fn run_test(input_path: String, result_path: String) {
        let input = fs::read_to_string(&input_path).expect("could not load file");
        let result = fs::read_to_string(&result_path).expect("could not load file");
        assert_eq!(compile(&input), result);
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
            Err(e) => eprintln!("{}", e),
        }
    }
}
