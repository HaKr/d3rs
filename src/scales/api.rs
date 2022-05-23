use std::{
    fmt::{Debug, Display},
    ops::{AddAssign, Sub},
};

pub type Result<T> = std::result::Result<T, ScaleError>;

pub trait DomainScale<DT>
where
    DT: PartialEq + PartialOrd + Debug + Display + Copy + Sub<DT, Output = DT> + AddAssign<DT>,
{
    fn domain_to_coordinate(&self, codomain: DT) -> Option<usize>;
    fn coordinate_to_domain(&self, coordinate: usize) -> Option<DT>;
}

#[derive(Debug)]
pub enum ScaleError {
    DimensionTooSmall,
    OutOfRange { explain: String },
    RangeExceedsMaximum { explain: String },
}

impl std::error::Error for ScaleError {}

impl Display for ScaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleError::OutOfRange { explain } => {
                f.write_fmt(format_args!("Out of range: {}", explain))
            }
            ScaleError::RangeExceedsMaximum { explain } => {
                f.write_fmt(format_args!("Range too large {}", explain))
            }
            ScaleError::DimensionTooSmall => f.write_str("Dimension is too small."),
        }
    }
}
