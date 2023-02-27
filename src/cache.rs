use chrono::{DateTime, Utc};
use std::time::SystemTime;
use std::collections::HashMap;

pub struct Cache<'a> {
    pub entries: HashMap<String, CacheEntry<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct CacheEntry<'a> {
    pub created_at: SystemTime,
    pub value: &'a [u8],
}

impl<'a> Cache<'a> {

    pub fn new(&mut self, cache: CacheEntry<'a>, interval: SystemTime) {

        let mut expired_entries: Vec<String> = Vec::new();
        for (key, entry) in self.entries.iter() {
            if entry.created_at > interval {
                let entry_key = key;
                expired_entries.push(entry_key.to_string());
            }
        }

        for entry in expired_entries {
            self.entries.remove_entry(&entry);
        }

        self.entries.insert("2".to_string(), cache);

    }
    
    pub fn add(&mut self, key: String, value: CacheEntry<'a>) {
        self.entries.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> (Option<&CacheEntry<'_>>, bool) {
        if self.entries.contains_key(&key) {
            (self.entries.get(&key).clone(), true)
        } else {
            (None, false)
        }
    }
//rustc -Z unstable-options --pretty expanded hello.rs
    pub fn read_loop(&mut self) {
        todo!("Read loop. Create an entry in cache removing any cache older than that (in time)")
    }
}
