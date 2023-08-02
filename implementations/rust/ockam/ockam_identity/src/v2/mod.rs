#![allow(missing_docs, dead_code)]

mod error;

pub use error::*;

/// On-the-wire data types
pub mod models;

/// Service for the management of identities
pub mod identities;

/// Data types representing an identity
pub mod identity;

/// Purpose keys
pub mod purpose_keys;

/// Purpose key
pub mod purpose_key;

///
/// Exports
///
pub use identities::*;
pub use identity::*;
pub use purpose_key::*;
pub use purpose_keys::*;
