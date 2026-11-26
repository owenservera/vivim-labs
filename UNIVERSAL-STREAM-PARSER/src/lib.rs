pub mod browser;
pub mod framing;
pub mod locate;
pub use framing::{Format, Parser, Record};
pub use locate::{Goal, Hit, Locator};
pub mod intelligence;
