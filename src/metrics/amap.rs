use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl std::fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                value.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}

impl AmapMetrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let map = metrics_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();

        Self {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> anyhow::Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!(format!("key {} not found", key)))?;
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    pub fn dec(&self, key: impl AsRef<str>) -> anyhow::Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or(anyhow::anyhow!(format!("key {} not found", key)))?;
        counter.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}
