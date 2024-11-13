use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .data
            .read()
            .map_err(|_| std::fmt::Error)?
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");
        writeln!(f, "{}", data)
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to lock"))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> anyhow::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to lock"))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|_| anyhow::anyhow!("Failed to lock"))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
