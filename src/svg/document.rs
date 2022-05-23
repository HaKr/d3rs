use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

const XMLNS: &str = "http://www.w3.org/2000/svg";
// const XLINK: &str = "xmlns:xlink=\"http://www.w3.org/1999/xlink\"";

#[derive(Default, Debug)]
#[xml_element("svg")]
pub struct Document {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_attr(rename = "viewBox")]
    view_box: String,

    #[sxs_type_attr]
    xmlns: &'static str,

    #[sxs_type_element]
    style: Option<CSS>,

    #[sxs_type_multi_element]
    items: Vec<XMLElement>,
}

#[derive(Debug, Default)]
#[xml_element("style")]
struct CSS {
    #[sxs_type_text]
    text: String,
}

global_attributes!(Document);

has_children!(Document);

impl Document {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            id: None,
            class: None,
            meta: None,
            view_box: format!("0 0 {} {}", width, height),
            xmlns: XMLNS,
            style: None,
            items: vec![],
        }
    }

    pub fn with_css<CT>(mut self, style: CT) -> Self
    where
        CT: Display,
    {
        self.style = Some(CSS {
            text: format!("{}", style),
        });

        self
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xml = XMLElement::from(self);

        f.write_str(xml.to_string_pretty("\n", "\t").as_str())
    }
}
