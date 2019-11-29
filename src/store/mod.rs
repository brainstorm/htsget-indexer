use crate::indexer::TargetRange;
use crate::errors::Result;

#[cfg(feature = "csv")]
pub mod csv;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "db")]
pub mod db;

pub trait Store {
  fn open(&mut self, resource: &str) -> Result<()>;
  fn save(&mut self, bam_id: &str, target_name: &str, range: &TargetRange) -> Result<()>;
}
