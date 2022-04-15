#[allow(unused_imports)] // suppress the warning about PI
use std::{
    f64::consts::PI,
    fmt::{Debug, Display},
    ops::{AddAssign, Range, Sub},
    slice::Iter,
};

#[derive(Debug, PartialEq)]
pub struct ScaledStep<'ss, DOMAIN> {
    value: &'ss DOMAIN,
    dimension: usize,
}

pub struct ScaledSteps<DOMAIN = u16> {
    dimension: usize,
    values: Vec<DOMAIN>,
    dimension_step: usize,
    dimension_start: usize,
}

pub struct ScaledStepsIter<'ssi, DOMAIN> {
    dimension: usize,
    dimension_step: usize,
    iter: Iter<'ssi, DOMAIN>,
}

impl<'ssi, DOMAIN> ScaledSteps<DOMAIN> {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            values: Vec::new(),
            dimension_step: 1,
            dimension_start: 0,
        }
    }

    pub fn iter(&'ssi self) -> ScaledStepsIter<'ssi, DOMAIN> {
        ScaledStepsIter {
            dimension: self.dimension_start,
            dimension_step: self.dimension_step,
            iter: self.values.iter(),
        }
    }
}

impl<N> ScaledSteps<N>
where
    N: Copy + AddAssign<N>,
{
    fn assign_steps(mut self, count: i128, start_value: N, step_value: N) -> Self {
        let mut current_value = start_value;

        self.values = (0..count).fold(Vec::new(), |mut values, _| {
            values.push(current_value);
            current_value += step_value;

            values
        });

        self
    }
}
impl<N> ScaledSteps<N>
where
    N: Copy
        + Sized
        + Into<i128>
        + From<i32>
        + PartialOrd
        + Sub<N, Output = N>
        + AddAssign<N>
        + Default,
{
    pub fn discrete_range(mut self, r: Range<N>) -> Self {
        let value_distance: i128 = (r.end - r.start).into();
        let sign = if r.start > r.end { -1 } else { 1 };
        let value_distance_abs = i128::abs(value_distance);

        let dimension = self.dimension as i128;

        let (dimension_step, domain_step) = if dimension > value_distance_abs {
            (dimension / value_distance_abs, N::from(sign))
        } else {
            let rem = value_distance % dimension != 0;
            let step = (value_distance / dimension) + if rem { sign as i128 } else { 0 };
            (1, N::from(step as i32))
        };

        self.dimension_step = dimension_step as usize;
        self.dimension_start = 0;

        let domain_step_i128: i128 = domain_step.into();

        let rem = value_distance % domain_step_i128 != 0;
        let count = (value_distance / domain_step_i128) + if rem { 1 } else { 0 };

        self.assign_steps(count, r.start, domain_step)
    }
}

impl ScaledSteps<f64> {
    pub fn continuous_range(mut self, r: Range<f64>) -> Self {
        let value_distance = r.end - r.start;

        let dimension = self.dimension as f64;

        let domain_step = value_distance / dimension;

        self.dimension_step = 1;

        let count = f64::floor(value_distance / domain_step) as i128;

        self.assign_steps(count, r.start, domain_step)
    }
}

impl<DOMAIN> ScaledSteps<DOMAIN>
where
    DOMAIN: Display,
{
    pub fn ordered<I>(mut self, steps: I) -> Self
    where
        I: IntoIterator<Item = DOMAIN>,
    {
        let max_index = self.dimension - 1;

        self.values = steps
            .into_iter()
            .enumerate()
            .filter_map(|(value_index, domain_value)| {
                if value_index < max_index {
                    Some(domain_value)
                } else {
                    None
                }
            })
            .fold(Vec::new(), |mut values, domain_value| {
                values.push(domain_value);

                values
            });

        self.dimension_step = self.dimension / (self.values.len() + 1);
        self.dimension_start = self.dimension_step;

        self
    }
}

impl<'ssi, DOMAIN> Iterator for ScaledStepsIter<'ssi, DOMAIN> {
    type Item = ScaledStep<'ssi, DOMAIN>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dom) = self.iter.next() {
            let result = ScaledStep {
                dimension: self.dimension,
                value: dom,
            };
            self.dimension += self.dimension_step;

            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn empty() {
    let scaled_steps = ScaledSteps::<i8>::new(800);
    let mut scaled_steps_iter = scaled_steps.iter();
    assert_eq!(scaled_steps_iter.next(), None);
}

#[cfg(test)]
fn assert_last<N>(scaled_steps: ScaledSteps<N>, count: usize, last_dimension: usize, last_value: N)
where
    N: Debug + PartialEq,
{
    assert_eq!(scaled_steps.iter().count(), count);
    assert_eq!(
        scaled_steps.iter().last(),
        Some(ScaledStep {
            dimension: last_dimension,
            value: &last_value
        })
    );
}
#[test]
fn angles_in_degrees() {
    assert_last(ScaledSteps::new(800).discrete_range(0..360), 360, 718, 359);
}

#[test]
fn angles_in_f64_degrees() {
    assert_last(
        ScaledSteps::new(800).continuous_range(0.0..360.0),
        800,
        799,
        359.5499999999947,
    );
}

#[test]
fn angles_in_f64_degrees_reversed_and_negative() {
    assert_last(
        ScaledSteps::new(800).continuous_range(180.0..-180.0),
        800,
        799,
        -179.54999999999853,
    );
}

#[test]
fn angles_in_f64_degrees_reversed_and_negative_some_rounding() {
    assert_last(
        ScaledSteps::new(360).continuous_range(180.0..-180.0),
        360,
        359,
        -179.0,
    );
}

#[test]
fn angles_in_negative_degrees() {
    assert_last(
        ScaledSteps::new(730).discrete_range(-180..181),
        361,
        720,
        180,
    );
}

#[test]
fn angles_in_negative_degrees_reversed() {
    assert_last(
        #[allow(clippy::reversed_empty_ranges)] // we use range.start and range.end only
        ScaledSteps::new(730).discrete_range(180..-181),
        361,
        720,
        -180,
    );
}

#[test]
fn angles_in_degrees_on_small_scale() {
    assert_last(
        ScaledSteps::new(100).discrete_range(0..360),
        360 / 4,
        89,
        356,
    );
}

#[test]
fn angles_in_degrees_on_half_scale() {
    assert_last(ScaledSteps::new(180).discrete_range(0..360), 180, 179, 358);
}

#[test]
fn just_over_divisor() {
    assert_last(ScaledSteps::new(181).discrete_range(0..360), 180, 179, 358);
}

#[test]
fn just_under_divisor() {
    assert_last(ScaledSteps::new(179).discrete_range(0..360), 120, 119, 357);
}

#[test]
fn large_domain_values() {
    assert_last(
        ScaledSteps::new(600).discrete_range(0..22_000_000_000_i64),
        600,
        599,
        21_963_333_533,
    );
}

#[test]
fn angles_in_radians() {
    let end = 2.0 * PI;
    assert_last(
        ScaledSteps::new(360).continuous_range(0.0..end),
        360,
        359,
        6.265732014659597,
    );
}

#[test]
fn angles_in_radians_also_negative() {
    assert_last(
        ScaledSteps::new(360).continuous_range(-PI..PI),
        360,
        359,
        3.124139361069829,
    );
}

#[test]
fn angles_in_reversed_radians_also_negative() {
    assert_last(
        ScaledSteps::new(360).continuous_range(PI..-PI),
        360,
        359,
        -3.124139361069829,
    );
}

#[test]
fn steps_over_domain() {
    assert_last(
        ScaledSteps::new(600).ordered((1967..2024).rev().step_by(7)),
        9,
        (600 / 10) * 9,
        1967,
    );
}

#[test]
fn steps_over_labeled_domain() {
    assert_last(
        ScaledSteps::new(600).ordered(vec!["Alpha", "Beta", "Gamma", "Delta", "Epsilon"]),
        5,
        500,
        "Epsilon",
    );
}

#[test]
fn steps_over_labeled_domain_with_too_small_dimension() {
    assert_last(
        ScaledSteps::new(5).ordered(vec!["Alpha", "Beta", "Gamma", "Delta", "Epsilon"]),
        4,
        4,
        "Delta",
    );
}

#[test]
fn steps_over_string_labeled_domain_() {
    assert_last(
        ScaledSteps::new(50).ordered(
            vec!["Alpha", "Beta", "Gamma", "Delta", "Epsilon"]
                .iter()
                .map(|s| String::from(*s)),
        ),
        5,
        (50 / 6) * 5,
        String::from("Epsilon"),
    );
}
