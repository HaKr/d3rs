use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct RenderError {}

impl Error for RenderError {}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The was a rendering error!")
    }
}
