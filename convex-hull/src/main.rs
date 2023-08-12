use solver::Solver;

mod solver;
mod fs;

#[allow(dead_code)]
fn main() {
    let mut solver = Solver::new();
    match solver.load_points_from_file("./res/test.txt") {
        Ok(points) => println!("{:?}", points),
        Err(msg) => println!("{msg}"),
    }
}
