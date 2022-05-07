use d3rs::scales::*;

fn main() {
    let x_axis_degrees = Linear::try_new(0, 360_u16, 10).unwrap();

    for (domain_x, coord_x) in x_axis_degrees.iter() {
        println!("coordinate={}, domain={}", coord_x, domain_x);
    }

    let x_axis_degrees = Linear::try_new(-180, 180_i16, 10).unwrap();

    for (domain_x, coord_x) in x_axis_degrees.iter() {
        println!("coordinate={}, domain={}", coord_x, domain_x);
    }

    let x_axis_degrees = Linear::try_new(180, -180_i16, 10).unwrap();

    for (domain_x, coord_x) in x_axis_degrees.iter() {
        println!("coordinate={}, domain={}", coord_x, domain_x);
    }

    let x_axis_degrees = Linear::try_new(0, 360_u16, 12).unwrap();

    for (domain_x, coord_x) in x_axis_degrees.iter() {
        println!("coordinate={}, domain={}", coord_x, domain_x);
    }
}
