use crate::indexer::TargetRange;
use crate::errors::Result;

#[cfg(feature = "csv")]
pub mod csv;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "database")]
pub mod db;
pub mod schema;
pub mod models;

pub trait Store {
  fn save<T, Q>(&mut self, bam_id: T, target_name: Q, range: &TargetRange) -> Result<()>;
}
