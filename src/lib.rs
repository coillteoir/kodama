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
    pub fn to_obj_string(self) -> String {
        let mut result = String::from("f ");

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

fn vertex_string(points: Vec<Point>) -> String {
    points
        .into_iter()
        .map(Point::to_obj_string)
        .collect::<Vec<String>>()
        .join("\n")
}


fn render_obj(points: Vec<Point>, faces: Vec<Face>) -> String {
    format!(
        r#"{0}
{1}
"#,
        vertex_string(points.to_vec()),
        faces
            .into_iter()
            .map(Face::to_obj_string)
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn sphere(origin: Point, radius: f32, _detail: u32) -> Result<String, String> {
    // top and bottom point
    // create vertical "strips" down the sphere,
    // rotating around the Z axis
    // Create faces procedurally

    let points = [
        //top
        Point::new(origin.x, origin.y + (radius * (2.0 * PI).sin()), origin.z),
        //bottom
        Point::new(origin.x, origin.y + (radius * (-2.0 * PI).sin()), origin.z),
        //north
        Point::new(origin.x, origin.y, origin.z + (radius * (2.0 * PI).sin())),
        //south
        Point::new(origin.x, origin.y, origin.z + (radius * (-2.0 * PI).sin())),
        //east
        Point::new(origin.x + (radius * (0.0_f32).cos()), origin.y, origin.z),
        //west
        Point::new(origin.x + (radius * (PI).cos()), origin.y, origin.z),
    ]
    .to_vec();

    let faces = [
        // top, north, east
        Face::new(points.clone(), vec![0, 2, 4]),
        // top, north, west
        Face::new(points.clone(), vec![0, 2, 5]),
        // top, south, east
        Face::new(points.clone(), vec![0, 3, 4]),
        // top, south, west
        Face::new(points.clone(), vec![0, 3, 5]),
        // bottom, north, east
        Face::new(points.clone(), vec![1, 2, 4]),
        // bottom, north, west
        Face::new(points.clone(), vec![1, 2, 5]),
        // bottom, south, east
        Face::new(points.clone(), vec![1, 3, 4]),
        // bottom, south, west
        Face::new(points.clone(), vec![1, 3, 5]),
    ];

    Ok(render_obj(points, faces.to_vec()))
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

    render_obj(points.to_vec(), faces.to_vec())
}

fn cuboid(origin: Point, sx: f32, sy: f32, sz: f32) -> Result<String, String> {
    println!("GENERATING CUBOID");
    if sx <= 0.0 || sy <= 0.0 || sz <= 0.0 {
        return Err(
            String::from("could not generate cuboid, side length less than or equal to zero"),
        );
    }
    let points = [
        Point::new(origin.x, origin.y + sy, origin.z + sz),
        Point::new(origin.x, origin.y, origin.z + sz),
        Point::new(origin.x + sx, origin.y, origin.z + sz),
        Point::new(origin.x + sx, origin.y + sy, origin.z + sz),
        Point::new(origin.x, origin.y + sy, origin.z),
        Point::new(origin.x, origin.y, origin.z),
        Point::new(origin.x + sx, origin.y, origin.z),
        Point::new(origin.x + sx, origin.y + sy, origin.z),
    ];

    let faces = [
        Face::new(points.to_vec(), vec![0, 1, 2, 3]),
        Face::new(points.to_vec(), vec![7, 6, 5, 4]),
        Face::new(points.to_vec(), vec![4, 5, 1, 0]),
        Face::new(points.to_vec(), vec![3, 7, 4, 0]),
        Face::new(points.to_vec(), vec![3, 2, 6, 7]),
        Face::new(points.to_vec(), vec![6, 2, 1, 5]),
    ];

    Ok(render_obj(points.to_vec(), faces.to_vec()))
}

fn cube(origin: Point, size: f32) -> Result<String, String> {
    println!("GENERATING CUBE");
    if size <= 0.0 {
        return Err("ERROR: cannot generate cube of size less than zero".to_string());
    }
    cuboid(origin, size, size, size)
}

pub fn compile(data: &str) -> String {
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
