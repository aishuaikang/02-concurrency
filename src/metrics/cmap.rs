use std::{fmt::Display, sync::Arc};

use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl Display for CmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
    }

    pub fn dec(&self, key: impl Into<String>) {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
