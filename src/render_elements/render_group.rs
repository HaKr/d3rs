use crate::{RenderElement, RenderStyle};

pub struct RenderGroup<'rg> {
    origin: (usize, usize),
    style: Option<&'rg RenderStyle>,
}

impl<'rg> RenderGroup<'rg> {
    pub fn new(origin: (usize, usize)) -> Self {
        Self {
            origin,
            style: None,
        }
    }

    pub fn with_style(mut self, render_style: &'rg RenderStyle) -> Self {
        self.style = Some(render_style);

        self
    }
}

impl<'rg> RenderElement<'rg> for RenderGroup<'rg> {
    fn origin(&self) -> &(usize, usize) {
        &self.origin
    }
}
