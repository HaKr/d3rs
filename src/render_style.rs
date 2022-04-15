use std::collections::{btree_map::Iter, BTreeMap};

enum RenderStyles {
    Inline(BTreeMap<String, String>),
    Class(String),
}

pub const CLASS: &str = "class";

pub struct RenderStyle {
    styles: RenderStyles,
}

pub struct StyleIter<'i> {
    iter: Option<Iter<'i, String, String>>,
    class: Option<&'i str>,
}

impl RenderStyle {
    pub fn styles<I: IntoIterator<Item = (&'static str, &'static str)>>(styles: I) -> Self {
        Self {
            styles: RenderStyles::Inline(styles.into_iter().fold(
                BTreeMap::new(),
                |mut attrs, (attr_key, attr_value)| {
                    attrs.insert(String::from(attr_key), String::from(attr_value));

                    attrs
                },
            )),
        }
    }

    pub fn attributes(&self) -> StyleIter {
        match &self.styles {
            RenderStyles::Inline(inline) => StyleIter::new(Some(inline.iter()), None),
            RenderStyles::Class(class) => StyleIter::new(None, Some(class.as_str())),
        }
    }
}

impl Default for RenderStyle {
    fn default() -> Self {
        Self {
            styles: RenderStyles::Inline(BTreeMap::new()),
        }
    }
}

impl From<&str> for RenderStyle {
    fn from(class: &str) -> Self {
        Self {
            styles: RenderStyles::Class(String::from(class)),
        }
    }
}

impl<'i> StyleIter<'i> {
    pub fn new(iter: Option<Iter<'i, String, String>>, class: Option<&'i str>) -> Self {
        Self { iter, class }
    }
}
impl<'i> Iterator for StyleIter<'i> {
    type Item = (&'i str, &'i str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.iter {
            if let Some((key, value)) = iter.next() {
                Some((key.as_str(), value.as_str()))
            } else {
                None
            }
        } else {
            self.class.map(|class| (CLASS, class))
        }
    }
}

#[test]
fn styles() {
    let s1 = RenderStyle::from("view");
    assert_eq!(s1.attributes().next(), Some(("class", "view")));

    let s2 = RenderStyle::styles(vec![("font-style", "sans"), ("fill", "0x12345")]);
    let mut s2_attrs = s2.attributes();
    assert_eq!(s2_attrs.next(), Some(("fill", "0x12345")));
    assert_eq!(s2_attrs.next(), Some(("font-style", "sans")));
    assert_eq!(s2_attrs.next(), None);
}
