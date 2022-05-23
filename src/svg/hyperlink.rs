use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("a")]
#[derive(Default)]
pub struct Hyperlink {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_attr]
    href: String,

    #[sxs_type_attr]
    target: Option<HyperlinkTarget>,

    #[sxs_type_multi_element]
    items: Vec<XMLElement>,
}

pub enum HyperlinkTarget {
    _Self,
    Parent,
    Top,
    Blank,
    Name(String),
}

global_attributes!(Hyperlink);

has_children!(Hyperlink);

impl Hyperlink {
    pub fn new<D>(href: D) -> Self
    where
        D: Display,
    {
        Self {
            href: format!("{}", href),
            ..Self::default()
        }
    }

    pub fn with_target(mut self, target: HyperlinkTarget) -> Self {
        self.target = Some(target);

        self
    }
}

impl Display for HyperlinkTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HyperlinkTarget::_Self => "_self",
            HyperlinkTarget::Parent => "_parent",
            HyperlinkTarget::Top => "_top",
            HyperlinkTarget::Blank => "_blank",
            HyperlinkTarget::Name(name) => name.as_str(),
        };

        f.write_str(s)
    }
}
