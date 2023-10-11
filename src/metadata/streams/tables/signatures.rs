#[cfg(feature = "read")]
#[doc(inline)]
pub use read::*;
#[cfg(feature = "write")]
#[doc(inline)]
pub use write::*;

#[cfg(feature = "read")]
#[doc(hidden)]
mod read {}

#[cfg(feature = "write")]
#[doc(hidden)]
mod write {}
