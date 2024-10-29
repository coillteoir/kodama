use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn to_obj_string(&self) -> String {
        format!("v {0} {1} {2}", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
struct Face<'lt> {
    points: &'lt [Point],
    indexes: &'lt [u32],
}

impl<'lt> Face<'lt> {
    pub fn new(points: &'lt [Point], indexes: &'lt [u32]) -> Self {
        Self { points, indexes }
    }

    pub fn to_obj_string(&self) -> String {
        let mut result = String::from("f ");

        result.push_str(
            &self
                .indexes
                .iter()
                .map(|i: &u32| -> String {
                    format!("-{0}", (self.points.len() as u64 - u64::from(*i)))
                })
                .collect::<Vec<String>>()
                .join(" "),
        );
        result
    }
}

fn vertex_string(points: &[Point]) -> String {
    points
        .iter()
        .map(Point::to_obj_string)
        .collect::<Vec<String>>()
        .join("\n")
}

fn render_obj(points: &[Point], faces: &[Face]) -> String {
    format!(
        r#"{0}
{1}
"#,
        vertex_string(points),
        faces
            .iter()
            .map(|f: &Face| Face::to_obj_string(f))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn sphere(origin: Point, radius: f32, _detail: u32) -> Result<String, String> {
    // top and bottom point
    // create vertical "strips" down the sphere,
    // rotating around the Z axis
    // Create faces procedurally
    // To start we'll create a simple octahedron

    let top = Point::new(origin.x, origin.y + (radius * (PI / 2.).sin()), origin.z);
    let bottom = Point::new(origin.x, origin.y + (radius * (PI / -2.).sin()), origin.z);
    let north = Point::new(origin.x, origin.y, origin.z + (radius * (PI / 2.).sin()));
    let south = Point::new(origin.x, origin.y, origin.z + (radius * (PI / -2.).sin()));
    let east = Point::new(origin.x + (radius * (0.0_f32).cos()), origin.y, origin.z);
    let west = Point::new(origin.x + (radius * (PI).cos()), origin.y, origin.z);

    let points = [top, bottom, north, south, east, west];

    let faces = [
        // top, north, east
        Face::new(&points, &[0, 2, 4]),
        // top, north, west
        Face::new(&points, &[0, 2, 5]),
        // top, south, east
        Face::new(&points, &[0, 3, 4]),
        // top, south, west
        Face::new(&points, &[0, 3, 5]),
        // bottom, north, east
        Face::new(&points, &[1, 2, 4]),
        // bottom, north, west
        Face::new(&points, &[1, 2, 5]),
        // bottom, south, east
        Face::new(&points, &[1, 3, 4]),
        // bottom, south, west
        Face::new(&points, &[1, 3, 5]),
    ];

    Ok(render_obj(&points, &faces))
}

pub fn cone(origin: Point, _detail: i32, _radius: f32) -> String {
    let points = [
        Point::new(origin.x, origin.y + (PI / 3.0).sin(), origin.z),
        Point::new(origin.x, origin.y, origin.z + 0.0_f32.cos()),
        Point::new(
            origin.x + (4. * PI / 3.).sin(),
            origin.y,
            (2. * -PI / 3.).cos(),
        ),
        Point::new(
            origin.x + (2. * PI / 3.).sin(),
            origin.y,
            (2. * -PI / 3.).cos(),
        ),
    ];

    let faces = [
        Face::new(&points, &[1, 2, 3]),
        Face::new(&points, &[0, 2, 3]),
        Face::new(&points, &[0, 1, 3]),
        Face::new(&points, &[0, 2, 1]),
    ];

    render_obj(&points, &faces)
}

pub fn cuboid(origin: Point, sx: f32, sy: f32, sz: f32) -> Result<String, String> {
    if sx <= 0. || sy <= 0. || sz <= 0. {
        return Err(String::from(
            "could not generate cuboid, side length less than or equal to zero",
        ));
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
        Face::new(&points, &[0, 1, 2, 3]),
        Face::new(&points, &[7, 6, 5, 4]),
        Face::new(&points, &[4, 5, 1, 0]),
        Face::new(&points, &[3, 7, 4, 0]),
        Face::new(&points, &[3, 2, 6, 7]),
        Face::new(&points, &[6, 2, 1, 5]),
    ];

    Ok(render_obj(&points, &faces))
}

pub fn cube(origin: Point, size: f32) -> Result<String, String> {
    if size <= 0.0 {
        return Err("ERROR: cannot generate cube of size less than zero".to_string());
    }
    cuboid(origin, size, size, size)
}
