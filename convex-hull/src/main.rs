use solver::Solver;

mod solver;
mod fs;

fn main() {
    let solver = Solver::new();
    let _ = solver.load_points_from_file("./res/test.txt");
}
