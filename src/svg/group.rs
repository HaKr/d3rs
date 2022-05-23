use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

#[xml_element("g")]
#[derive(Default)]
pub struct Group {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_multi_element]
    items: Vec<XMLElement>,
}

global_attributes!(Group);

has_children!(Group);
