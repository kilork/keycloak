#[cfg(all(feature = "reqwest12", not(feature = "reqwest13")))]
pub use reqwest12 as reqwest;
#[cfg(all(feature = "reqwest12", feature = "reqwest13"))]
pub use reqwest13 as reqwest;
#[cfg(all(feature = "reqwest13", not(feature = "reqwest12")))]
pub use reqwest13 as reqwest;
