use std::{fmt::Display, hash::Hash, ops::AddAssign};

#[derive(Default)]
pub struct CategorisedValue<PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub primary_key: PT,
    pub secondary_key: ST,
    pub value: VT,
}

impl<PT> From<PT> for CategorisedValue<PT, usize, usize>
where
    PT: Display + Hash + Eq,
{
    fn from(definition: PT) -> Self {
        CategorisedValue::new(definition, 0, 1)
    }
}

impl<PT, VT> From<(PT, VT)> for CategorisedValue<PT, usize, VT>
where
    PT: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    fn from(definition: (PT, VT)) -> Self {
        CategorisedValue::new(definition.0, 0, definition.1)
    }
}

impl<PT, ST, VT> From<(PT, ST, VT)> for CategorisedValue<PT, ST, VT>
where
    PT: Clone + Default + Display + Hash + Eq,
    ST: Clone + Default + Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    fn from(definition: (PT, ST, VT)) -> Self {
        CategorisedValue::new(definition.0, definition.1, definition.2)
    }
}

impl<PT, ST, VT> CategorisedValue<PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub fn new(primary_key: PT, secondary_key: ST, value: VT) -> Self {
        Self {
            primary_key,
            secondary_key,
            value,
        }
    }
}
