use crate::prelude::parse::loader::IncludeLoader;

#[derive(Debug, Default)]
pub struct NoopIncludeLoader;

impl IncludeLoader for NoopIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, String> {
        Err(format!("unable to resolve {path:?}"))
    }
}
