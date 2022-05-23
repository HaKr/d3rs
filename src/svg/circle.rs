use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

use crate::LengthOrPercentage;

#[xml_element("circle")]
#[derive(Default)]
pub struct Circle {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_attr]
    pub cx: LengthOrPercentage,

    #[sxs_type_attr]
    pub cy: LengthOrPercentage,

    #[sxs_type_attr]
    pub r: LengthOrPercentage,
}

global_attributes!(Circle);

impl Circle {
    pub fn new(cx: LengthOrPercentage, cy: LengthOrPercentage, r: LengthOrPercentage) -> Self {
        Self {
            cx,
            cy,
            r,
            ..Self::default()
        }
    }
}
