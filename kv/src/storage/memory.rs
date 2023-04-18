use crate::{KvError, Kvpair, Storage, StorageIter, Value};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用DashMap 构建的MemTable，实现了Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key.into(), value.into()))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    /*
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // 使用clone() 来获取table的snapshot
        let table = self.get_or_create_table(table).clone();
        let iter = table.into_iter().map(|data| data.into());
        Ok(Box::new(iter))
    }
    */

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // 使用clone()来获取table的snapshot
        let table = self.get_or_create_table(table).clone();
        let iter = StorageIter::new(table.into_iter());
        Ok(Box::new(iter))
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(data: (String, Value)) -> Self {
        Kvpair::new(data.0, data.1)
    }
}
