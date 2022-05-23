use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Sub},
};

use super::{DomainScale, Result, ScaleError};

pub trait IterableScale<DT>
where
    DT: PartialEq + PartialOrd + Debug + Display + Copy + Sub<DT, Output = DT> + AddAssign<DT>,
{
    fn iter(&self) -> DomainIter<DT>;
    fn intervals(&self, step: DT) -> DomainIter<DT>;
}

#[derive(Debug)]
pub struct Linear<DT>
where
    DT: PartialEq + PartialOrd + Debug + Display + Copy + Sub<DT, Output = DT>,
{
    start: DT,
    dimension: usize,
    min: DT,
    max: DT,
    ratio: f64,
    domain_range: f64,
}

#[derive(Debug)]
pub struct DomainIter<DT>
where
    DT: Debug + Copy + AddAssign<DT> + Sub<DT, Output = DT>,
{
    current: f64,
    domain_step: f64,
    increment: f64,
    dimension: f64,
    dimension_end: usize,
    from_float: fn(f64) -> DT,
}

impl<DT> Linear<DT>
where
    DT: PartialEq
        + PartialOrd
        + Debug
        + Display
        + Copy
        + Add<DT, Output = DT>
        + AddAssign<DT>
        + Sub<DT, Output = DT>,
    Self: ConvertToFloat<DT>,
{
    pub fn try_new(start: DT, end: DT, dimension: usize) -> Result<Self> {
        if dimension < 5 {
            return Err(ScaleError::DimensionTooSmall);
        }

        let (min, max, sign) = if start < end {
            (start, end, 1.0_f64)
        } else {
            (end, start, -1.0_f64)
        };
        if min < <Self as ConvertToFloat<DT>>::MIN {
            return Err(ScaleError::OutOfRange {
                explain: format!(
                    "minimum value {} is out of range; must be larger than {}",
                    min,
                    <Self as ConvertToFloat<DT>>::MIN
                ),
            });
        }
        if max > <Self as ConvertToFloat<DT>>::MAX {
            return Err(ScaleError::OutOfRange {
                explain: format!(
                    "maximum value {} is out of range; must be less than {}",
                    max,
                    <Self as ConvertToFloat<DT>>::MAX
                ),
            });
        }

        let domain_range = max - min;
        if domain_range > Self::MAX {
            return Err(ScaleError::OutOfRange {
                explain: format!(
                    "Difference between {} and {} is out of range; must be less than {}",
                    min,
                    max,
                    <Self as ConvertToFloat<DT>>::MAX
                ),
            });
        }
        let domain_range = Self::to_float(domain_range);
        let ratio = domain_range / (dimension - 1) as f64 * sign;

        if f64::is_infinite(ratio) {
            return Err(ScaleError::DimensionTooSmall);
        }

        Ok(Self {
            start,
            dimension,
            min,
            max,
            ratio,
            domain_range,
        })
    }

    fn create_iter(&self, step: f64) -> DomainIter<DT> {
        let increment = f64::abs(step / self.ratio);

        let domain_step = increment * self.ratio;
        DomainIter {
            current: <Self as ConvertToFloat<DT>>::to_float(self.start),
            domain_step,
            increment,
            dimension: 0.0,
            dimension_end: self.dimension,
            from_float: <Self as ConvertToFloat<DT>>::from_float,
        }
    }
}

impl<DT> Iterator for DomainIter<DT>
where
    DT: Debug + Copy + PartialOrd + AddAssign<DT> + Sub<DT, Output = DT>,
{
    type Item = (DT, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let dimension = f64::round(self.dimension) as usize;
        if dimension < self.dimension_end {
            let result = ((self.from_float)(self.current), dimension);
            self.dimension += self.increment;
            self.current += self.domain_step;
            Some(result)
        } else {
            None
        }
    }
}

pub trait ConvertToFloat<DT>
where
    DT: PartialEq + PartialOrd + Display + Copy + Sub<DT, Output = DT>,
{
    const MIN: DT;
    const MAX: DT;
    const ADJUST: DT;
    const ZERO: DT;

    fn to_float(domain: DT) -> f64;
    fn from_float(float: f64) -> DT;
}

macro_rules! implement_numerical_traits {
    ($typ:ty, $adjust:literal, $min:expr, $max:expr ) => {
        impl ConvertToFloat<$typ> for Linear<$typ> {
            const ZERO: $typ = (0 as $typ);
            const ADJUST: $typ = $adjust;
            const MIN: $typ = $min;
            const MAX: $typ = $max;

            fn to_float(domain: $typ) -> f64 {
                domain as f64
            }

            fn from_float(float: f64) -> $typ {
                float as $typ
            }
        }

        impl DomainScale<$typ> for Linear<$typ> {
            fn domain_to_coordinate(&self, value: $typ) -> Option<usize> {
                if self.min <= value && value <= self.max {
                    let diff = (if value < self.start {
                        self.start - value
                    } else {
                        value - self.start
                    }) as f64;
                    let dimension =
                        f64::round((diff / self.domain_range) * (self.dimension - 1) as f64)
                            as usize;

                    Some(dimension)
                } else {
                    None
                }
            }

            fn coordinate_to_domain(&self, coordinate: usize) -> Option<$typ> {
                if coordinate < self.dimension {
                    let domain_value = self.ratio * (coordinate as f64);
                    Some(self.start + domain_value as $typ)
                } else {
                    None
                }
            }
        }

        impl IterableScale<$typ> for Linear<$typ> {
            fn iter(&self) -> DomainIter<$typ> {
                self.create_iter(self.ratio)
            }

            fn intervals(&self, step: $typ) -> DomainIter<$typ> {
                self.create_iter(<Self as ConvertToFloat<$typ>>::to_float(step))
            }
        }
    };
}

implement_numerical_traits!(i64, 1, -9_007_199_254_740_991, 9_007_199_254_740_990);
implement_numerical_traits!(i32, 1, i32::MIN, i32::MAX - 1);
implement_numerical_traits!(i16, 1, i16::MIN, i16::MAX - 1);
implement_numerical_traits!(usize, 1, 0, 9_007_199_254_740_990);
implement_numerical_traits!(u32, 1, u32::MIN, u32::MAX - 1);
implement_numerical_traits!(u16, 1, u16::MIN, u16::MAX - 1);
implement_numerical_traits!(f32, 0.0, f32::MIN, f32::MAX);
implement_numerical_traits!(f64, 0.0, f64::MIN, f64::MAX);

#[cfg(test)]
fn show_result<DT>(scale: Result<Linear<DT>>)
where
    DT: PartialEq + PartialOrd + Debug + Display + Copy + Sub<DT, Output = DT>,
{
    match scale {
        Ok(scale) => println!("linear={:?}", scale),
        Err(err) => println!("Failed to create: {}", err),
    }
}

#[test]
fn numerical() {
    show_result(Linear::try_new(360_i64, -360, 300));
    show_result(Linear::try_new(
        -9_007_199_254_740_990_i64,
        9_007_199_254_740_990,
        1,
    ));
    show_result(Linear::try_new(7000, 9_007_199_254_740_995_i64, 1));
    show_result(Linear::try_new(0_u16, 65533, 10));
    show_result(Linear::try_new(-1.0_f32, 1.0, 200));
}

#[test]
fn intervals() {
    // let linear = Linear::try_new(360_i16, -360, 300).unwrap();

    // for tick in linear.intervals(15).enumerate() {
    //     println!("Tick={:?}", tick);
    // }

    let linear = Linear::try_new(-std::f64::consts::PI, std::f64::consts::PI, 360).unwrap();

    for tick in linear.iter().enumerate() {
        println!("Tick={:?}, sin(x)={}", tick, f64::sin(tick.1 .0));
    }
}
