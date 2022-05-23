use std::time::Instant;

use d3rs::scales::{DomainScale, IterableScale, Linear, ScaleError};

fn main() -> Result<(), ScaleError> {
    let start = Instant::now();
    const WIDTH: usize = 121;
    const CONSOLE_WIDTH: usize = WIDTH + 1;
    const HEIGHT: usize = 24;
    let mut grid = [[' '; CONSOLE_WIDTH]; HEIGHT];
    let setup_elapsed = start.elapsed().as_micros();
    let x_axis_radians = Linear::try_new(0.0_f64, 2.0 * std::f64::consts::PI, WIDTH)?;
    let x_axis_degrees = Linear::try_new(0, 360_u16, WIDTH)?;
    let y_axis = Linear::try_new(1_f64, -1.0, HEIGHT)?;

    let axis_time = start.elapsed().as_micros();

    for (radians, coord_x) in x_axis_radians.intervals(std::f64::consts::FRAC_PI_8) {
        let sin_x = f64::sin(radians);
        let coord_y = y_axis.domain_to_coordinate(sin_x).unwrap() as usize;
        let deg = format!("{}", x_axis_degrees.coordinate_to_domain(coord_x).unwrap());
        let deg_str = deg.as_bytes();
        let mut coord_str = coord_x - if deg.len() > 2 { 1 } else { 0 };
        for d in deg_str.iter() {
            grid[coord_y][coord_str] = *d as char;
            coord_str += 1;
        }
    }

    let draw_time = start.elapsed().as_micros();

    let s = grid.map(|r| r.iter().collect::<String>()).join("\n");
    println!("{}", s);

    println!(
        "Setup={}us, {}, {}, axis={}us, draw={}us",
        setup_elapsed,
        axis_time,
        draw_time,
        axis_time - setup_elapsed,
        draw_time - axis_time
    );

    Ok(())
}
