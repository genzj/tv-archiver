pub mod glob_source_collector;

pub use crate::metadata::Metadata;

type CollectResult = Result<Vec<Metadata>, String>;

pub trait SourceCollector {
    fn collect(&self, source: &String) -> CollectResult;
}
