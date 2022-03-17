pub use ffi;
pub use glib;


#[allow(unused_imports)]
mod auto;
mod builder;
pub mod prelude;
mod enums;

pub use crate::auto::*;
pub use crate::builder::IndicatorBuilder;
pub use crate::enums::*;