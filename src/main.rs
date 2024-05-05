use std::f32::consts::PI;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
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
                .map(|i: u32| -> String { format!("-{0}", (self.points.len() as i32 - i as i32)) })
                .collect::<Vec<String>>()
                .join(" "),
        );
        result
    }
}

fn main() {
    println!("{}", compile("cuboid 2 3 4".to_string()).unwrap())
}

fn vertex_string(points: Vec<Point>) -> String {
    points
        .into_iter()
        .map(|p| p.to_obj_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn _sphere(origin: Point, radius: f32, _detail: u32) -> Result<String, String> {
    if radius <= 0.0 {
        return Err("radius less than zero".to_string());
    }
    // Initialize top and bottom points
    let _points = [origin];
    Ok("".to_string())
}

fn prism(origin: Point, _radius: f32) -> Result<String, String> {
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

    //    println!(
    //        "{}",
    //        faces
    //            .clone()
    //            .into_iter()
    //            .map(|f| f.to_obj_string())
    //            .collect::<Vec<String>>()
    //            .join("\n")
    //    );

    Ok(format!(
        r#"{0}
{1}
"#,
        vertex_string(points.to_vec(),),
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

fn compile(data: String) -> Result<String, String> {
    let mut result = String::from("");

    let lines = data.split('\n');
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens[0] {
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
            "prism" => result.push_str(
                &prism(
                    Point::new(0.0, 0.0, 0.0),
                    tokens[1].parse::<f32>().expect("non numeric value given"),
                )
                .unwrap(),
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
            Err(e) => eprintln!("{}", e),
        }
    }
}
