use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum LengthOrPercentage {
    Number(isize),
    Em(isize),
    Ex(isize),
    Pixels(isize),
    Inch(f32),
    Cm(f32),
    Mm(f32),
    Point(usize),
    Pica(usize),
    Percentage(f32),
    Raw(String),
}

impl Default for LengthOrPercentage {
    fn default() -> Self {
        Self::ZERO
    }
}

impl LengthOrPercentage {
    pub const ZERO: LengthOrPercentage = LengthOrPercentage::Number(0);
    pub const HUNDRED_PERCENT: LengthOrPercentage = LengthOrPercentage::Percentage(100.0);
    pub const ONE_QUARTER: LengthOrPercentage = LengthOrPercentage::Percentage(25.0);
    pub const ONE_THIRD: LengthOrPercentage = LengthOrPercentage::Percentage(33.3333);
    pub const TWO_THIRD: LengthOrPercentage = LengthOrPercentage::Percentage(66.6667);
    pub const HALF: LengthOrPercentage = LengthOrPercentage::Percentage(50.0);
    pub const THREE_QUARTER: LengthOrPercentage = LengthOrPercentage::Percentage(75.0);
    pub const TEN_PERCENT: LengthOrPercentage = LengthOrPercentage::Percentage(10.0);
}

impl LengthOrPercentage {
    pub fn new<T>(raw: T) -> Self
    where
        T: Display,
    {
        Self::Raw(format!("{}", raw))
    }
}

impl Display for LengthOrPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthOrPercentage::Number(num) => f.write_fmt(format_args!("{}", num)),
            LengthOrPercentage::Em(em) => f.write_fmt(format_args!("{}em", em)),
            LengthOrPercentage::Ex(ex) => f.write_fmt(format_args!("{}ex", ex)),
            LengthOrPercentage::Pixels(px) => {
                f.write_fmt(format_args!("{}{}", px, if *px > 0 { "px" } else { "" }))
            }
            LengthOrPercentage::Inch(inch) => f.write_fmt(format_args!("{}in", inch)),
            LengthOrPercentage::Cm(cm) => f.write_fmt(format_args!("{}cm", cm)),
            LengthOrPercentage::Mm(mm) => f.write_fmt(format_args!("{}mm", mm)),
            LengthOrPercentage::Point(pt) => f.write_fmt(format_args!("{}pt", pt)),
            LengthOrPercentage::Pica(pc) => f.write_fmt(format_args!("{}pc", pc)),
            LengthOrPercentage::Percentage(perc) => write!(f, "{}%", perc),
            LengthOrPercentage::Raw(raw) => f.write_str(raw),
        }
    }
}
