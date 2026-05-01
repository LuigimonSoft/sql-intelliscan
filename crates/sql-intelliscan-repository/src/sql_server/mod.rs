//! SQL Server repository implementations.
//!
//! This module is the only backend boundary that may depend on `mssqlrust`.
//! Upper layers must stay driver-agnostic and interact with SQL Server behavior
//! through repository contracts instead of importing driver types directly.

mod connection_repository;
mod metadata_repository;

pub use connection_repository::SqlServerConnectionRepository;
pub use metadata_repository::SqlServerMetadataRepository;
