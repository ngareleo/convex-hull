use std::fs;

// coordinates range from 0 to 100

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn as_tuple(self: &Self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn from_txt_line(line: &str) -> Result<Self, String> {
        let xy: Vec<&str> = line.trim().split(",").collect();
        let x = xy[0].parse().map_err(|_| format!("Invalid X coordinate given. xy is {:?}", xy))?;
        let y = xy[1].parse().map_err(|_| format!("Invalid Y coordinate given. xy is {:?}", xy))?;
        Ok(Point::new(x, y))
    }
}

pub struct Solver {}

impl Solver {
    pub fn new() -> Self {
        Solver {}
    }

    pub fn load_points_from_file(self: &Solver, file_path: &str) -> Result<Vec<Point>, String> {
        let content = fs
            ::read_to_string(file_path)
            .map_err(|e| format!("Error opening file. Message {e}"))?;
        let lines = content
            .split("\n")
            .filter(|line| -> bool {
                !line.starts_with("//") &&
                    line.contains(",") &&
                    line.split(",").collect::<Vec<&str>>().len() == 2
            })
            .collect::<Vec<&str>>();
        let mut points: Vec<Point> = Vec::with_capacity(lines.len());
        for (i, line) in lines.iter().enumerate() {
            let point = Point::from_txt_line(line).map_err(|e| {
                format!("Error parsing file on line {}. Message : {}", i + 1, e)
            })?;
            points.push(point);
        }
        Ok(points)
    }
}
