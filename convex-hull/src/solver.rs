use std::fs;

// coordinates range from 0 to 100

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn from_txt_line(line: &str) -> Option<Self> {
        if line.starts_with("//") {
            return None;
        }
        let xy: Vec<&str> = line.trim().split(",").collect();
        println!("{:?}", xy);
        match xy.len() {
            2 => Some(Point::new(xy[0].parse().unwrap(), xy[1].parse().unwrap())),
            _ => None,
        }
    }
}

pub struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Solver {}
    }

    pub fn load_points_from_file(self: &Solver, file_path: &str) {
        let content = fs::read_to_string(file_path).expect("Error reading file");
        let points: &Vec<Option<Point>> = &content
            .split("\n")
            .map(|val| Point::from_txt_line(val))
            .filter(|val| val.is_some())
            .collect();

        for (i, point) in points.iter().enumerate() {
            println!("{i}: {:?}", point);
        }
    }
}
