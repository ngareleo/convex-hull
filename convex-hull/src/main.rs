use solver::{ Solver, Point };
use raylib::prelude::*;

mod solver;
mod fs;

#[allow(dead_code)]
fn main() {
    let (width, height) = (500, 500);

    let mut solver = Solver::new();
    match solver.load_points_from_file("./res/test.txt") {
        Ok(_) => println!("{:?}", solver.points),
        Err(msg) => println!("{msg}"),
    }

    let _ = solver.solve();

    let (mut rl, thread) = raylib::init().size(width, height).title("Convex hull").build();
    rl.set_target_fps(20);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        match &solver.points {
            Some(points) => {
                for point in points {
                    d.draw_circle(point.x + width / 2, point.y + height / 2, 3.0, Color::RED);
                }
            }
            None => {}
        }
    }
}
