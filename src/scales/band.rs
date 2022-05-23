use std::fmt::{Debug, Display};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Band<DT>
where
    DT: PartialEq + PartialOrd + Debug + Display,
{
    dimension: usize,
    domain: Vec<DT>,
    padding_inner: f64,
    padding_outer: f64,
    align: f64,
}

pub struct BandIter<'i, DT>
where
    DT: PartialEq + PartialOrd + Debug + Display,
{
    step: f64,
    current: f64,
    bandwidth: usize,
    iter: std::slice::Iter<'i, DT>,
}

fn calculate_dimension(n: usize, dimension: usize, padding_inner: f64) -> usize {
    usize::max(
        dimension,
        f64::round((n - 1) as f64 / padding_inner) as usize,
    )
}

impl<DT> Band<DT>
where
    DT: PartialEq + PartialOrd + Debug + Display + Default,
{
    pub fn new<I>(domain: I, dimension: usize) -> Self
    where
        I: IntoIterator<Item = DT>,
    {
        let domain: Vec<DT> = domain.into_iter().collect();
        let dimension = calculate_dimension(domain.len(), dimension, 0.1);

        Self {
            dimension,
            domain,
            padding_inner: 0.1,
            padding_outer: 0.05,
            align: 0.5,
        }
    }

    pub fn padding_inner(mut self, padding: f64) -> Self {
        self.padding_inner = if padding < 1.0 { padding } else { 0.1 };
        self.dimension = calculate_dimension(self.domain.len(), self.dimension, self.padding_inner);

        self
    }

    pub fn iter<'i>(&'i self) -> BandIter<'i, DT> {
        let n = self.domain.len() as f64;
        // step = (stop - start) / Math.max(1, n - paddingInner + paddingOuter * 2);
        //  let computed_step = n as f32 - self.padding_inner + self.padding_outer * 2f32;
        let step = self.dimension as f64
            / f64::max(1.0, n - self.padding_inner + self.padding_outer * 2.0);
        // start += (stop - start - step * (n - paddingInner)) * align;
        let current = ((self.dimension - 1) as f64 - step * (n - self.padding_inner)) * self.align;
        // bandwidth = step * (1 - paddingInner);
        let bandwidth = usize::max(f64::round(step * (1.0 - self.padding_inner)) as usize, 1);
        let iter = self.domain.iter();

        BandIter {
            bandwidth,
            current,
            step,
            iter,
        }
    }
}

impl<'i, DT> Iterator for BandIter<'i, DT>
where
    DT: PartialEq + PartialOrd + Debug + Display + Default,
{
    type Item = (&'i DT, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(domain) = self.iter.next() {
            let dimension_start = f64::round(self.current) as usize;
            let dimension_end = dimension_start + self.bandwidth - 1;

            let result = (domain, (dimension_start, dimension_end));

            self.current += self.step;

            Some(result)
        } else {
            None
        }
    }
}

#[test]
fn create_band() {
    let band = Band::new(1977..2018, 600).padding_inner(0.1);

    for (domain, (start, end)) in band.iter() {
        println!("domain: {} -> ({}, {})", domain, start, end)
    }

    let band = Band::new(vec!["Apples", "Pears", "Bananas"], 300);

    for (domain, (start, end)) in band.iter() {
        println!("domain: {} -> ({}, {})", domain, start, end)
    }
}
