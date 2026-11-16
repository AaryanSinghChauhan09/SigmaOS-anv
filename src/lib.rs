// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-anv: anv component model.
//!
//! Real implementation: anv_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnvItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnvError {
    EmptyAnvItemName,
    DuplicateAnvItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAnvItemName => write!(f, "anv_item name must not be empty"),
            Self::DuplicateAnvItem(id) => write!(f, "duplicate anv_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AnvError {}

pub struct Anv {
    pub name: String,
    anv_items: BTreeMap<String, AnvItem>,
    events: BTreeMap<String, Event>,
}

impl Anv {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            anv_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_anv_item(&mut self, item: AnvItem) -> Result<(), AnvError> {
        if item.name.trim().is_empty() {
            return Err(AnvError::EmptyAnvItemName);
        }
        let id = item.id.clone();
        if self.anv_items.contains_key(&id) {
            return Err(AnvError::DuplicateAnvItem(id.clone()));
        }
        self.anv_items.insert(id, item);
        Ok(())
    }

    pub fn anv_item_count(&self) -> usize {
        self.anv_items.len()
    }

    pub fn get_anv_item(&self, id: &str) -> Option<&AnvItem> {
        self.anv_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AnvError> {
        if event.subject.trim().is_empty() {
            return Err(AnvError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_anv_item(id: &str, name: &str) -> AnvItem {
        AnvItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Anv::new("Anv");
        assert_eq!(m.name, "Anv");
        assert_eq!(m.anv_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_anv_item() {
        let mut m = Anv::new("Anv");
        m.add_anv_item(sample_anv_item("a1", "Alpha")).unwrap();
        assert_eq!(m.anv_item_count(), 1);
        assert!(m.get_anv_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Anv::new("Anv");
        assert!(m.add_anv_item(sample_anv_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Anv::new("Anv");
        m.add_anv_item(sample_anv_item("x", "One")).unwrap();
        assert!(m.add_anv_item(sample_anv_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Anv::new("Anv");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
