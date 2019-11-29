use crate::indexer::TargetRange;
use crate::errors::Result;

#[cfg(feature = "csv")]
pub mod csv;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "database")]
pub mod db;

pub trait Store {
  fn save(&mut self, bam_id: &str, target_name: &str, range: &TargetRange) -> Result<()>;
}
