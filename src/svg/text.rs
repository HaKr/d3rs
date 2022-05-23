use std::fmt::Display;

use simple_xml_serialize::XMLElement;
use simple_xml_serialize_macro::xml_element;

use crate::LengthOrPercentage;

#[xml_element("text")]
#[derive(Default)]
pub struct Text {
    #[sxs_type_attr]
    id: Option<String>,

    #[sxs_type_attr]
    class: Option<String>,

    #[sxs_type_attr(rename = "data-meta")]
    meta: Option<String>,

    #[sxs_type_attr]
    x: Option<LengthOrPercentage>,

    #[sxs_type_attr]
    y: Option<LengthOrPercentage>,

    #[sxs_type_attr]
    dx: Option<LengthOrPercentage>,

    #[sxs_type_attr]
    dy: Option<LengthOrPercentage>,

    #[sxs_type_attr]
    rotate: Option<u16>,

    #[sxs_type_text]
    text: String,
}

global_attributes!(Text);

impl Text {
    pub fn new<D>(text: D) -> Self
    where
        D: Display,
    {
        Self {
            text: format!("{}", text),
            ..Self::default()
        }
    }

    /// The coordinates of the starting point of the text baseline
    pub fn at(mut self, x: LengthOrPercentage, y: LengthOrPercentage) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self.dx = None;
        self.dy = None;

        self
    }

    /// Shifts the text position relative to a previous text element
    pub fn relative(mut self, dx: LengthOrPercentage, dy: LengthOrPercentage) -> Self {
        self.dx = Some(dx);
        self.dy = Some(dy);
        self.x = None;
        self.y = None;

        self
    }
}
