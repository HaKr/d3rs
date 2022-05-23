use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

use crate::LengthOrPercentage;

#[xml_element("line")]
#[derive(Default)]
pub struct Line {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_attr]
    x1: LengthOrPercentage,

    #[sxs_type_attr]
    y1: LengthOrPercentage,

    #[sxs_type_attr]
    x2: LengthOrPercentage,

    #[sxs_type_attr]
    y2: LengthOrPercentage,
}

global_attributes!(Line);

impl Line {
    pub fn new(
        x1: LengthOrPercentage,
        y1: LengthOrPercentage,
        x2: LengthOrPercentage,
        y2: LengthOrPercentage,
    ) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            ..Self::default()
        }
    }

    pub fn from(mut self, x: LengthOrPercentage, y: LengthOrPercentage) -> Self {
        self.x1 = x;
        self.y1 = y;

        self
    }

    pub fn to(mut self, x: LengthOrPercentage, y: LengthOrPercentage) -> Self {
        self.x2 = x;
        self.y2 = y;

        self
    }
}
