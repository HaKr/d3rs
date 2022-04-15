use crate::{Result, View};

/// A chart is basically just a Euclidian coordinate system, where it's views can define elements upon
///
///
///
///
#[derive(Default)]
pub struct Chart<'c> {
    views: Vec<&'c dyn View>,
}

impl<'c> Chart<'c> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_view(mut self, view: &'c dyn View) -> Self {
        self.views.push(view);

        self
    }

    pub fn render(&self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn create_chart() -> Result<()> {
    let chart = Chart::new();
    let _result = chart.render()?;
    // assert_eq!(result, ());
    Ok(())
}
