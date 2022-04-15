mod render_error;
pub use render_error::RenderError as Error;

pub type Result<T> = std::result::Result<T, Error>;

mod render_style;
pub use render_style::RenderStyle;

mod render_element;
pub use render_element::RenderElement;

mod chart;
pub use chart::Chart;

mod view;
pub use view::View;

pub mod render_elements;
pub use render_elements as elements;

mod continuous_mapper;
pub use continuous_mapper::*;

pub mod data_collections;
