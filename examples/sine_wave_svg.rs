#![feature(trace_macros)]
#![feature(log_syntax)]

use std::{fs::File, io::Write};

use d3rs::{Document, DomainScale, Group, IterableScale, LengthOrPercentage, Linear, Result};

fn main() -> Result<()> {
    const BORDER_WIDTH_LEFT: usize = 30;
    const BORDER_WIDTH_RIGHT: usize = 15;
    const HEIGHT: usize = 640;
    const BORDER_HEIGHT: usize = 20;
    const WAVE_WIDTH: usize = 2 * 720;
    const WAVE_HEIGHT: usize = HEIGHT - BORDER_HEIGHT * 2;
    const WIDTH: usize = WAVE_WIDTH + BORDER_WIDTH_LEFT + BORDER_WIDTH_RIGHT;
    const PLOT_THICKNESS: usize = 2;

    const TWO_PI: f64 = 3.0 * std::f64::consts::PI;

    let x_axis_radians = Linear::try_new(0.0_f64, TWO_PI, WAVE_WIDTH)?;
    let x_axis_degrees = Linear::try_new(0, 540_u16, WAVE_WIDTH)?;
    let y_axis = Linear::try_new(1.3_f64, -1.3, WAVE_HEIGHT)?;

    let mut out = File::create("sine.svg").unwrap();
    write!(
        out,
        "{}",
        d3rs::svg!(
            WIDTH,
            HEIGHT,
            style:
                format!(
                    r#"
            .tangent circle {{
                fill: red;
            }}

            .sine circle {{
                fill: green;
            }}

            .cosine circle {{
                fill: blue;
            }}

            .axis line {{
                stroke: black;
            }}

            .horizontal-ticks, .chart {{
                transform: translate( {transx}px, {transy}px );
            }}

            .vertical-ticks {{
                transform: translate( 0, {transy}px );
            }}

            .vertical-ticks text {{
                text-anchor: end;
            }}

            .horizontal-ticks text {{
                text-anchor: middle;
            }}

            .tick-label {{
                font-size: 8pt;
            }}

            line.tick {{
                stroke: grey;
            }}
        "#,
                    transx = BORDER_WIDTH_LEFT,
                    transy = BORDER_HEIGHT
                ),
            [
                d3rs::group!( class: "axis", [

                    d3rs::horizontal_axis!(
                        ( 0, "50%" ), ( "100%", "50%" ),
                        x_axis_degrees.intervals(15),
                        position: WAVE_HEIGHT / 2,
                        tick_label: |deg,_| if deg >0 {Some(deg)} else {None},
                        label_y_offset: 10
                    ),

                    d3rs::vertical_axis!(
                        ( BORDER_WIDTH_LEFT, 0 ), ( BORDER_WIDTH_LEFT, "100%" ),
                        y_axis.intervals(0.1),
                        position: BORDER_WIDTH_LEFT,
                        tick_label: |y, _| {
                            if f64::round(f64::abs(y) * 11.0) > 0.0 {
                                Some(format!("{:>4.1}", y))
                            } else {
                                None
                            }
                        },
                        label_x_offset: -1,
                        label_y_offset: 4
                    )
                ]),
                d3rs::group!( class: "chart", [
                    d3rs::plot!(x_axis_radians.iter(), |radians, coord_x| {
                        let tan_x = f64::tan(radians);
                        let deg: f64 = (radians * 360.0) / TWO_PI;
                        if let Some(coord_y) = y_axis.domain_to_coordinate(tan_x) {

                        Some(d3rs::circle!(
                            (coord_x, coord_y),
                            PLOT_THICKNESS,
                            meta: format!("angle={:08.3};tanx={:5.4}", deg, tan_x)
                        ))
                    } else {
                        None
                    }
                    }, class: "tangent"),
                    d3rs::plot!(x_axis_radians.iter(), |radians, coord_x| {
                        let sin_x = f64::sin(radians);
                        let deg: f64 = (radians * 360.0) / TWO_PI;
                        let coord_y = y_axis.domain_to_coordinate(sin_x).unwrap() as usize;

                        Some(d3rs::circle!(
                            (coord_x, coord_y),
                            PLOT_THICKNESS,
                            meta: format!("angle={:08.3};sinx={:5.4}", deg, sin_x)
                        ))
                    }, class: "sine"),
                    d3rs::plot!(x_axis_radians.iter(), |radians, coord_x| {
                        let cos_x = f64::cos(radians);
                        let deg: f64 = (radians * 360.0) / TWO_PI;
                        let coord_y = y_axis.domain_to_coordinate(cos_x).unwrap() as usize;

                        Some(d3rs::circle!(
                            (coord_x, coord_y),
                            PLOT_THICKNESS,
                            meta: format!("angle={:08.3};cosx={:5.4}", deg, cos_x)
                        ))
                    }, class: "cosine")
                ])
            ]
        )
    )
    .unwrap();

    Ok(())
}
