use crate::RenderStyle;

pub trait RenderElement<'re> {
    fn origin(&'re self) -> &'re (usize, usize);

    fn style(&self) -> Option<RenderStyle> {
        None
    }
}
