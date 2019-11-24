use crate::indexer::TargetRange;
use crate::errors::Result;

#[cfg(feature = "csv")]
pub mod csv;

#[cfg(feature = "json")]
pub mod json;

pub trait Store {
  fn save(&mut self, target_name: &str, range: &TargetRange) -> Result<()>;
}
