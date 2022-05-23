use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

use std::{collections::HashMap, fmt::Display};

use crate::LengthOrPercentage;

#[derive(Debug, Default)]
#[xml_element("style")]
pub struct Styles {
    #[sxs_type_attr]
    media: Option<String>,
    #[sxs_type_text]
    rules: CSSRules,
}

#[derive(Debug, Default)]
pub struct CSSRules {
    rules: HashMap<String, Vec<Styling>>,
}

#[derive(Debug)]
pub struct Transform {
    functions: Vec<TransformFunction>,
}

#[derive(Debug)]
pub enum TransformFunction {
    Translate {
        x: LengthOrPercentage,
        y: LengthOrPercentage,
    },
    Rotate(Angle),
}

#[derive(Debug)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
    Turns(f32),
}

#[derive(Debug)]
pub enum ColorName {
    AliceBlue,
    Red,
    Green,
    Blue,
    Magenta,
    White,
    Black,
}

#[derive(Debug)]
pub enum Color {
    Name(ColorName),
    Rgb(Rgb),
    Hex(u32),
}

#[derive(Debug)]
pub enum ByteOrPercentage {
    Byte(u8),
    Percentage(f32),
}

#[derive(Debug)]
pub struct Rgb {
    red: ByteOrPercentage,
    green: ByteOrPercentage,
    blue: ByteOrPercentage,
    alpha: Option<f32>,
}

#[derive(Debug)]
pub enum Styling {
    Fill(Color),
    Stroke(Color),
    Transform(Transform),
    Raw(String),
}

impl Styles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn for_media(mut self, media: String) -> Self {
        self.media = Some(media);

        self
    }

    pub fn add_rule(mut self, selector: String, styling: Vec<Styling>) -> Self {
        self.rules.rules.insert(selector, styling);

        self
    }
}

impl Display for ColorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ColorName::AliceBlue => "aliceblue",
            ColorName::Red => "red",
            ColorName::Green => "green",
            ColorName::Blue => "blue",
            ColorName::Magenta => "magenta",
            ColorName::White => "white",
            ColorName::Black => "black",
        };

        f.write_str(name)
    }
}

impl ByteOrPercentage {
    pub fn number(b: u8) -> Self {
        Self::Byte(b)
    }

    pub fn percentage(p: f32) -> Self {
        Self::Percentage(force_valid_percentage(p))
    }
}

impl Display for ByteOrPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ByteOrPercentage::Byte(b) => f.write_fmt(format_args!("{}", b)),
            ByteOrPercentage::Percentage(p) => f.write_fmt(format_args!("{}%", p)),
        }
    }
}

impl Rgb {
    pub fn new(red: ByteOrPercentage, green: ByteOrPercentage, blue: ByteOrPercentage) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: None,
        }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(force_valid_percentage(alpha));

        self
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.alpha {
            Some(p) => f.write_fmt(format_args!(
                "rgb( {} {} {} / {}% )",
                self.red, self.green, self.blue, p
            )),
            None => f.write_fmt(format_args!(
                "rgb( {} {} {} )",
                self.red, self.green, self.blue
            )),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Name(name) => f.write_fmt(format_args!("{}", name)),
            Color::Rgb(rgba) => f.write_fmt(format_args!("{}", rgba)),
            Color::Hex(rgba) => {
                if *rgba < 0x01000000 {
                    f.write_fmt(format_args!("#{:06x}", rgba))
                } else {
                    f.write_fmt(format_args!("#{:08x}", rgba))
                }
            }
        }
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::Degrees(deg) => f.write_fmt(format_args!("{}deg", deg)),
            Angle::Radians(rad) => f.write_fmt(format_args!("{}rad", rad)),
            Angle::Turns(turns) => f.write_fmt(format_args!("{}turn", turns)),
        }
    }
}

impl Display for TransformFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformFunction::Translate { x, y } => {
                f.write_fmt(format_args!("translate( {}, {} )", x, y))
            }
            TransformFunction::Rotate(angle) => f.write_fmt(format_args!("rotate( {} )", angle)),
        }
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("transform:")?;
        for tfn in &self.functions {
            f.write_fmt(format_args!(" {}", tfn))?;
        }
        Ok(())
    }
}

impl Transform {
    pub fn new(transform: TransformFunction) -> Self {
        Self {
            functions: vec![transform],
        }
    }

    pub fn and_then(mut self, transform: TransformFunction) -> Self {
        self.functions.push(transform);

        self
    }
}

impl Display for Styling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Styling::Fill(fill_color) => f.write_fmt(format_args!("fill: {}", fill_color)),
            Styling::Stroke(stroke_color) => f.write_fmt(format_args!("stroke: {}", stroke_color)),
            Styling::Transform(transform) => f.write_fmt(format_args!("{}", transform)),
            Styling::Raw(raw) => f.write_str(raw),
        }
    }
}

impl Display for CSSRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (selector, rules) in &self.rules {
            f.write_fmt(format_args!("{} {{\n", selector))?;
            for rule in rules {
                f.write_fmt(format_args!("\t{};\n", rule))?;
            }
            f.write_str("}\n")?;
        }
        Ok(())
    }
}

#[inline]
fn force_valid_percentage(p: f32) -> f32 {
    f32::max(0.0, f32::min(p, 100.0))
}

#[test]
fn colors() {
    assert_eq!(Color::Name(ColorName::Red).to_string(), "red".to_owned());
    assert_eq!(
        Color::Name(ColorName::Green).to_string(),
        "green".to_owned()
    );
    assert_eq!(
        Color::Name(ColorName::AliceBlue).to_string(),
        "aliceblue".to_owned()
    );

    assert_eq!(Color::Hex(0x1234).to_string(), "#001234".to_owned());
    assert_eq!(Color::Hex(0x123456).to_string(), "#123456".to_owned());
    assert_eq!(Color::Hex(0x123400).to_string(), "#123400".to_owned());
    assert_eq!(Color::Hex(0x12300).to_string(), "#012300".to_owned());
    assert_eq!(Color::Hex(0x12345678).to_string(), "#12345678".to_owned());
    assert_eq!(Color::Hex(0x02345678).to_string(), "#02345678".to_owned());
    assert_eq!(Color::Hex(0x1234567).to_string(), "#01234567".to_owned());

    assert_eq!(
        Color::Rgb(Rgb::new(
            ByteOrPercentage::Byte(128),
            ByteOrPercentage::Byte(255),
            ByteOrPercentage::Percentage(25.0)
        ))
        .to_string(),
        "rgb( 128 255 25% )".to_owned()
    );
    assert_eq!(
        Color::Rgb(
            Rgb::new(
                ByteOrPercentage::Byte(128),
                ByteOrPercentage::Percentage(1.0),
                ByteOrPercentage::Byte(12)
            )
            .with_alpha(9999.0)
        )
        .to_string(),
        "rgb( 128 1% 12 / 100% )".to_owned()
    );
}

#[test]
fn transformations() {
    assert_eq!(
        Transform::new(TransformFunction::Translate {
            x: LengthOrPercentage::Cm(1.0),
            y: LengthOrPercentage::HALF
        })
        .to_string(),
        "transform: translate( 1cm 50% )".to_owned()
    );

    assert_eq!(
        Transform::new(TransformFunction::Rotate(Angle::Degrees(-60.0))).to_string(),
        "transform: rotate( -60deg )".to_owned()
    );

    assert_eq!(
        Transform::new(TransformFunction::Translate {
            x: LengthOrPercentage::Cm(1.0),
            y: LengthOrPercentage::HALF
        })
        .and_then(TransformFunction::Rotate(Angle::Degrees(-60.0)))
        .to_string(),
        "transform: translate( 1cm 50% ) rotate( -60deg )".to_owned()
    );
}
