use std::{
    collections::btree_map::Iter as BTreeMapIter,
    fmt::{Display, Write},
    hash::Hash,
    ops::{AddAssign, Index},
};

use indexmap::{
    map::Iter as IndexMapIter, set::Iter as IndexSetIter, Equivalent, IndexMap, IndexSet,
};

use super::{CategorisedValue, SegmentedValue};

#[derive(Default, Debug)]
/// Base for collecting values per category and optionally per segment
///
/// The values to collect are categorised by a key that must implement
/// the [Clone], [Default], [Display], [Hash] and [Eq] traits.
///
/// Optionally, each category can also be subdivided in segments.
/// The segment key must also implement the above mentioned traits.
///
/// The values must implement the [AddAssign], [Copy], [Default] and [Into]<[JsonValue]> traits.
///
/// # Example
/// ```rust
/// # use d3rs::data_collections::CategorisedValues;
///
/// let categorised = CategorisedValues::new()
///             // optionally define the order of the categories. The categories must have
///             // the same type as the ones in add_data
///            .with_primary_categories(1970..2000_i16)
///             // Also optional the order of the segments can be predefined.
///             // Again, the type of the segments must be equal to that used in add_data
///            .with_secondary_categories(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
///             // add some data....
///            .add_data(vec![
///                (1977, "Cassette", 36_900_000),
///                (1977_i16, "8 - Track", 127_300_000),
///                (1979, "8 - Track", 102_300_000),
///                (1978, "8 - Track", 133_600_000),
///                (1978, "Cassette", 61_300_000),
///                (1979, "Cassette", 78_500_000),
///            ])
///             // ...and some more. Calling add_data multiple times is allowed
///             // as long as the types of the categories, the segments and data values
///             // are the same in all calls.
///            .add_data(vec![
///                (2000, "CD", 942_500_000),
///                (2000, "DVD Audio", 1_000),
///                (2000, "Cassette", 76_000_000),
///                (2010, "DVD Audio", 40_000),
///                (2010, "CD", 253_000_000),
///            ]);
///
/// let expected = "{ 1970: {}, 1971: {}, 1972: {}, 1973: {}, 1974: {}, 1975: {}, 1976: {}, 1977: { 8 - Track: 127300000, Cassette: 36900000 }, 1978: { 8 - Track: 133600000, Cassette: 61300000 }, 1979: { 8 - Track: 102300000, Cassette: 78500000 }, 1980: {}, 1981: {}, 1982: {}, 1983: {}, 1984: {}, 1985: {}, 1986: {}, 1987: {}, 1988: {}, 1989: {}, 1990: {}, 1991: {}, 1992: {}, 1993: {}, 1994: {}, 1995: {}, 1996: {}, 1997: {}, 1998: {}, 1999: {}, 2000: { Cassette: 76000000, DVD Audio: 1000, CD: 942500000 }, 2010: { DVD Audio: 40000, CD: 253000000 } }";
///
/// assert_eq!(categorised.to_string(), expected );
/// ```
pub struct CategorisedValues<PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    secondary_categories: IndexSet<ST>,
    categorised_values: IndexMap<PT, SegmentedValue<VT>>,
}

pub struct PrimaryCategory<'sv, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub key: &'sv PT,
    categorised_values: &'sv CategorisedValues<PT, ST, VT>,
    segmented_values: &'sv SegmentedValue<VT>,
}

#[derive(Debug, PartialEq)]
pub struct SecondaryCategory<'sc, ST, VT>
where
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub key: &'sc ST,
    pub value: &'sc VT,
}

pub struct PrimaryCategoriesIter<'i, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    iter: IndexMapIter<'i, PT, SegmentedValue<VT>>,
    categorised_values: &'i CategorisedValues<PT, ST, VT>,
}

pub struct SecondaryCategoriesIter<'i, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    iter: BTreeMapIter<'i, usize, VT>,
    categorised_values: &'i CategorisedValues<PT, ST, VT>,
}

impl<PT, ST, VT> CategorisedValues<PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub fn new() -> Self {
        Self {
            secondary_categories: IndexSet::new(),
            categorised_values: IndexMap::new(),
        }
    }

    pub fn with_primary_categories<I: IntoIterator<Item = PT>>(
        mut self,
        primary_categories: I,
    ) -> Self {
        primary_categories.into_iter().for_each(|pc| {
            self.categorised_values
                .entry(pc)
                .or_insert_with(SegmentedValue::default);
        });

        self
    }

    pub fn with_secondary_categories<I: IntoIterator<Item = ST>>(
        mut self,
        secondary_categories: I,
    ) -> Self {
        self.secondary_categories.extend(secondary_categories);

        self
    }

    /// Add a collection of categorised data into this one
    ///
    /// The data comes from a collection that can be iterated over,
    /// where each item is something that supports the [Into]<[CategorisedValue]> trait.
    ///
    /// # Example
    /// ```rust
    /// # use d3rs::data_collections::CategorisedValues;
    ///
    /// let frequencies = CategorisedValues::new()
    ///        .with_primary_categories('a'..'z')
    ///        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));
    ///
    /// let expected = r#"{ "d": 1, "e": 1, "h": 1, "l": 3, "o": 2, "r": 1, "w": 1 }"#;
    ///
    /// //assert_eq!( frequencies.to_string(), expected );
    ///
    /// ```
    pub fn add_data<T: IntoIterator<Item = impl Into<CategorisedValue<PT, ST, VT>>>>(
        mut self,
        collection: T,
    ) -> Self {
        for def in collection.into_iter() {
            let bar_definition: CategorisedValue<PT, ST, VT> = def.into();
            let primary_key = bar_definition.primary_key;
            let (secondary_index, _) = self
                .secondary_categories
                .insert_full(bar_definition.secondary_key);
            self.categorised_values
                .entry(primary_key)
                .or_insert_with(SegmentedValue::default)
                .add(secondary_index, bar_definition.value);
        }

        self
    }

    pub fn iter<'i>(&'i self) -> PrimaryCategoriesIter<'i, PT, ST, VT> {
        PrimaryCategoriesIter {
            iter: self.categorised_values.iter(),
            categorised_values: &self,
        }
    }

    pub fn secondary_categories(&self) -> IndexSetIter<'_, ST> {
        self.secondary_categories.iter()
    }

    /// Closure that maps segment indices to their corresponding label value
    ///
    /// ```rust
    /// # use d3rs::data_collections::{CategorisedValues, SecondaryCategory} ;
    ///
    /// let categorised = CategorisedValues::new().add_data(vec![
    ///     ("A", "x", 11_u16),
    ///     ("B", "y", 13),
    ///     ("C", "z", 17),
    ///     ("A", "y", 19),
    ///     ("B", "z", 23),
    ///     ("C", "x", 29),
    ///     ("A", "z", 31),
    ///     ("B", "x", 37),
    ///     ("C", "y", 41),
    ///     ("A", "y", 43),
    /// ]);
    ///
    /// assert_eq!(
    ///     categorised
    ///         .iter()
    ///         .next()
    ///         .unwrap()
    ///         .values()
    ///         .skip(1)
    ///         .next()
    ///         .unwrap(),
    ///     SecondaryCategory{ key: &"y", value: &(19 + 43) }
    /// );
    /// ```
    pub fn map_secondary_index_to_key<'m>(&'m self) -> impl Fn(&usize) -> &'m ST + 'm {
        move |segment_index| self.secondary_index_to_key(*segment_index)
    }

    fn secondary_index_to_key(&self, segment_index: usize) -> &ST {
        &self.secondary_categories[segment_index]
    }
}

impl<PT, ST, VT, Q: ?Sized> Index<&Q> for CategorisedValues<PT, ST, VT>
where
    Q: Hash + Equivalent<PT>,
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    type Output = SegmentedValue<VT>;

    fn index(&self, index: &Q) -> &Self::Output {
        self.categorised_values
            .get(index)
            .expect("index out of bounds")
    }
}
//#[cfg(any(test, doctest))]
impl<PT, ST, VT> Display for CategorisedValues<PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let categories_count = self.categorised_values.len();
        let is_empty = categories_count < 1;
        let is_pretty = f.alternate();

        if is_empty {
            f.write_str("{}")
        } else {
            let values_only = self.secondary_categories.len() < 2;
            let last_index = categories_count - 1;

            f.write_str("{")?;
            f.write_char(if is_pretty { '\n' } else { ' ' })?;

            for (index, primary) in self.iter().enumerate() {
                if is_pretty {
                    f.write_char('\t')?
                }
                f.write_fmt(format_args!("{}: ", primary.key))?;

                if !values_only {
                    f.write_char('{')?;
                    if !primary.is_empty() {
                        f.write_char(' ')?
                    }
                }

                let mut write_seg_separator = false;
                for secondary in primary.values() {
                    if write_seg_separator {
                        f.write_str(", ")?;
                    } else {
                        write_seg_separator = true;
                    }

                    if !values_only {
                        f.write_fmt(format_args!("{}: ", secondary.key))?;
                    }

                    f.write_fmt(format_args!("{}", secondary.value))?;
                }

                if !values_only {
                    if !primary.is_empty() {
                        f.write_char(' ')?
                    }

                    f.write_char('}')?;
                }

                if index < last_index {
                    f.write_str(", ")?;
                }
                if is_pretty {
                    f.write_char('\n')?
                }
            }

            f.write_str(" }")
        }
    }
}

impl<'sv, PT, ST, VT> PrimaryCategory<'sv, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    pub fn is_empty(&self) -> bool {
        self.segmented_values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.segmented_values.len()
    }

    pub fn height(&self) -> VT {
        self.segmented_values.height()
    }

    pub fn values(&'sv self) -> SecondaryCategoriesIter<'sv, PT, ST, VT> {
        SecondaryCategoriesIter {
            iter: self.segmented_values.values(),
            categorised_values: self.categorised_values,
        }
    }
}

impl<'i, PT, ST, VT> Iterator for PrimaryCategoriesIter<'i, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    type Item = PrimaryCategory<'i, PT, ST, VT>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, values)) = self.iter.next() {
            Some(PrimaryCategory {
                categorised_values: self.categorised_values,
                segmented_values: values,
                key,
            })
        } else {
            None
        }
    }
}

impl<'i, PT, ST, VT> Iterator for SecondaryCategoriesIter<'i, PT, ST, VT>
where
    PT: Display + Hash + Eq,
    ST: Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    type Item = SecondaryCategory<'i, ST, VT>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((index, value)) = self.iter.next() {
            let key = self.categorised_values.secondary_index_to_key(*index);
            Some(SecondaryCategory { value, key })
        } else {
            None
        }
    }
}

#[cfg(test)]
fn assert_output_eq<PT, ST, VT>(categorised_values: CategorisedValues<PT, ST, VT>, json_str: &str)
where
    PT: Clone + Default + Display + Hash + Eq,
    ST: Clone + Default + Display + Hash + Eq,
    VT: AddAssign<VT> + Copy + Default + Display,
{
    assert_eq!(
        categorised_values
            .to_string()
            .replace('\n', "")
            .replace('\t', " "),
        String::from(json_str).replace('\n', "").replace("  ", " ")
    );
}

#[test]
fn empty_string_when_no_data() {
    assert_output_eq(CategorisedValues::<i8, i8, f32>::new(), "{}");
}

#[test]
fn categories_only() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![("C", 10_u16), ("B", 20), ("A", 30)]),
        "{ C: 10, B: 20, A: 30 }",
    )
}

#[test]
fn histogram() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec!["A", "B", "A", "C", "A", "C", "A", "A"]),
        "{ A: 5, B: 1, C: 2 }",
    )
}

#[test]
fn ordered_categories_only() {
    assert_output_eq(
        CategorisedValues::new()
            .with_primary_categories(vec!["B", "C", "A"])
            .add_data(vec![("C", 10_u16), ("B", 20), ("A", 30)]),
        "{ B: 20, C: 10, A: 30 }",
    );
}

#[test]
fn ordered_categories_with_multiple_entries() {
    assert_output_eq(
        CategorisedValues::new()
            .with_primary_categories(vec!["A", "B", "C"])
            .add_data(vec![
                ("C", 10_u16),
                ("B", 20),
                ("A", 30),
                ("A", 10),
                ("B", 20),
                ("C", 30),
            ]),
        "{ A: 40, B: 40, C: 40 }",
    );
}

#[test]
fn categories_with_segments() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![
            ("C", 12_u32, 10_u16),
            ("B", 10_u32, 20),
            ("A", 11_u32, 30),
        ]),
        "{ C: { 12: 10 }, B: { 10: 20 }, A: { 11: 30 } }",
    );
}

#[test]
fn categories_with_multiple_segments() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![
            ("A", "x", 11_u16),
            ("B", "y", 13),
            ("C", "z", 17),
            ("A", "y", 19),
            ("B", "z", 23),
            ("C", "x", 29),
            ("A", "z", 31),
            ("B", "x", 37),
            ("C", "y", 41),
            ("A", "y", 43),
        ]),
        "{ A: { x: 11, y: 62, z: 31 }, B: { x: 37, y: 13, z: 23 }, C: { x: 29, y: 41, z: 17 } }",
    );
}

#[test]
fn iterate_categories_and_segments() {
    let categorised = CategorisedValues::new()
        .with_primary_categories(1970..2000_i16)
        .with_secondary_categories(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
        .add_data(vec![
            (1977_i16, "Cassette", 36_900_000_i32),
            (1977, "8 - Track", 127_300_000),
            (1979, "8 - Track", 102_300_000),
            (1978, "8 - Track", 133_600_000),
            (1978, "Cassette", 61_300_000),
            (1979, "Cassette", 78_500_000),
        ])
        .add_data(vec![
            (2000_i16, "CD", 942_500_000),
            (2000, "DVD Audio", 1_000),
            (2000, "Cassette", 76_000_000),
            (2010, "DVD Audio", 40_000),
            (2010, "CD", 253_000_000),
        ]);

    let mut categories = categorised.iter().skip(7);

    let primary = categories.next().unwrap();
    assert_eq!(primary.key, &1977);
    assert_eq!(primary.height(), 36_900_000 + 127_300_000);
    assert!(!primary.is_empty());

    let mut segments_iter = primary.values();

    let secondary = segments_iter.next().unwrap();
    assert_eq!(secondary.value, &127_300_000);
    assert_eq!(secondary.key, &"8 - Track");

    let secondary = segments_iter.next().unwrap();
    assert_eq!(secondary.value, &36_900_000);
    assert_eq!(secondary.key, &"Cassette");

    assert_eq!(segments_iter.next(), None);

    let secondary = categories.next().unwrap();
    assert_eq!(secondary.height(), 133_600_000 + 61_300_000);
    assert_eq!(secondary.key, &1978);
    assert!(!secondary.is_empty());
}

#[test]
fn iterate_frequencies() {
    let categorised = CategorisedValues::new()
        .with_primary_categories('a'..'z')
        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));

    let secondary = &categorised[&'o'];
    assert_eq!(secondary.height(), 2);
    assert!(!secondary.is_empty());

    let category = categorised.iter().skip(11).next().unwrap();
    assert_eq!(category.key, &'l');
    assert_eq!(category.height(), 3);
    assert!(!category.is_empty());
}

#[test]
fn dbg() {
    let categorised = CategorisedValues::new()
        .with_primary_categories('a'..'z')
        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));

    println!("{}", categorised);
}

#[test]
fn to_string() {
    let categorised = CategorisedValues::new()
        .with_primary_categories(1970..2000_i16)
        .with_secondary_categories(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
        .add_data(vec![
            (1977_i16, "Cassette", 36_900_000_i32),
            (1977, "8 - Track", 127_300_000),
            (1979, "8 - Track", 102_300_000),
            (1978, "8 - Track", 133_600_000),
            (1978, "Cassette", 61_300_000),
            (1979, "Cassette", 78_500_000),
        ])
        .add_data(vec![
            (2000_i16, "CD", 942_500_000),
            (2000, "DVD Audio", 1_000),
            (2000, "Cassette", 76_000_000),
            (2010, "DVD Audio", 40_000),
            (2010, "CD", 253_000_000),
        ]);

    println!("{}", categorised);
}

#[test]
fn to_string_from_format() {
    assert_eq!(
        CategorisedValues::new()
            .add_data("hello world".chars().filter(|c| c.is_alphabetic()))
            .to_string(),
        String::from("{ h: 1, e: 1, l: 3, o: 2, w: 1, r: 1, d: 1 }")
    );

    assert_eq!(
		 CategorisedValues::new()
			 .with_secondary_categories(1..=1) // force two defined segments (0 added by data)
			 .add_data("hello world".chars().filter(|c| c.is_alphabetic()))
			 .to_string(),
		 String::from( "{ h: { 0: 1 }, e: { 0: 1 }, l: { 0: 3 }, o: { 0: 2 }, w: { 0: 1 }, r: { 0: 1 }, d: { 0: 1 } }" )
	 );
}
