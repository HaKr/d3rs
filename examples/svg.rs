use std::{fs::File, io::Write};

use d3rs::*;

fn main() {
    let mut doc = File::create("sample.svg").unwrap();

    let mut svg = Document::new(1920, 1080).with_css(
        r#"
        circle {
            fill: green;
            stroke: blue;
        }

        #🦀 {
            stroke: magenta;
        }

        .word {
            overflow-x: scroll;
            border: 5px solid #333333;
        }
    "#,
    );

    let circle = Circle::new(
        LengthOrPercentage::HALF,
        LengthOrPercentage::HALF,
        LengthOrPercentage::Percentage(100.0 / 6.0),
    )
    .with_class("test-only");

    let line = Line::new(
        LengthOrPercentage::Percentage(25.0),
        LengthOrPercentage::ZERO,
        LengthOrPercentage::ZERO,
        LengthOrPercentage::Percentage(75.0),
    )
    .with_class(999)
    .with_id('🦀');

    let mut link1 = Hyperlink::new("https://en.wikipedia.org/wiki/1974");

    let txt = Text::new("1974")
        .at(LengthOrPercentage::Cm(10.0), LengthOrPercentage::Point(12))
        .at(LengthOrPercentage::ZERO, LengthOrPercentage::Number(20));

    link1.add(txt);

    let mut group = Group::default();

    let mut link2 =
        Hyperlink::new("https://nos.nl").with_target(HyperlinkTarget::Name("right".to_owned()));

    link2.add(circle);
    group.add(line);
    group.add(link1);
    group.add(link2);

    svg.add(group);

    doc.write_fmt(format_args!("{}", svg)).unwrap();
}
