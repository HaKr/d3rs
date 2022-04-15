use std::{
    collections::{btree_map::Iter, BTreeMap},
    ops::AddAssign,
};

#[derive(Debug, Default)]
pub struct SegmentedValue<VAL>
where
    VAL: AddAssign<VAL> + Copy + Default,
{
    segments: BTreeMap<usize, VAL>,
    magnitude: VAL,
}

impl<VAL> SegmentedValue<VAL>
where
    VAL: AddAssign<VAL> + Copy + Default,
{
    pub fn add(&mut self, segment_index: usize, value: VAL) {
        self.magnitude += value;
        *self
            .segments
            .entry(segment_index)
            .or_insert_with(Default::default) += value;
    }

    pub fn value_of_segment(&self, segment_index: usize) -> Option<VAL> {
        self.segments.get(&segment_index).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn height(&self) -> VAL {
        self.magnitude
    }

    pub fn values(&self) -> Iter<'_, usize, VAL> {
        self.segments.iter()
    }
}
