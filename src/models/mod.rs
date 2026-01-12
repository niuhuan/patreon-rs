//! Patreon API data models.
//!
//! Patreon uses the JSON:API format. Responses generally follow this structure:
//! - `data`: primary resource data
//! - `included`: related resources
//! - `links`: pagination links
//! - `meta`: metadata

mod resources;
mod response;
mod attributes;
mod serde_helpers;

pub use resources::*;
pub use response::*;
pub use attributes::*;
