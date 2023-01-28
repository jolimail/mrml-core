use crate::prelude::hash::Map;
use crate::prelude::parse::loader::IncludeLoader;

#[derive(Debug, Default)]
pub struct MemoryIncludeLoader(pub Map<String, String>);

impl<K: ToString, V: ToString> From<Vec<(K, V)>> for MemoryIncludeLoader {
    fn from(value: Vec<(K, V)>) -> Self {
        let res = value
            .into_iter()
            .fold(Map::default(), |mut res, (key, value)| {
                res.insert(key.to_string(), value.to_string());
                res
            });
        MemoryIncludeLoader::from(res)
    }
}

impl From<Map<String, String>> for MemoryIncludeLoader {
    fn from(value: Map<String, String>) -> Self {
        MemoryIncludeLoader(value)
    }
}

impl IncludeLoader for MemoryIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, String> {
        self.0
            .get(path)
            .map(|v| v.clone())
            .ok_or_else(|| format!("unable to resolve {path:?}"))
    }
}
