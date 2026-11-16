// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-etc-passwd: etc passwd component model.
//!
//! Real implementation: passwds, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passwd {
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
pub enum EtcPasswdError {
    EmptyPasswdName,
    DuplicatePasswd(String),
    EmptyEventSubject,
}

impl fmt::Display for EtcPasswdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPasswdName => write!(f, "passwd name must not be empty"),
            Self::DuplicatePasswd(id) => write!(f, "duplicate passwd {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EtcPasswdError {}

pub struct EtcPasswd {
    pub name: String,
    passwds: BTreeMap<String, Passwd>,
    events: BTreeMap<String, Event>,
}

impl EtcPasswd {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passwds: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_passwd(&mut self, item: Passwd) -> Result<(), EtcPasswdError> {
        if item.name.trim().is_empty() {
            return Err(EtcPasswdError::EmptyPasswdName);
        }
        let id = item.id.clone();
        if self.passwds.contains_key(&id) {
            return Err(EtcPasswdError::DuplicatePasswd(id.clone()));
        }
        self.passwds.insert(id, item);
        Ok(())
    }

    pub fn passwd_count(&self) -> usize {
        self.passwds.len()
    }

    pub fn get_passwd(&self, id: &str) -> Option<&Passwd> {
        self.passwds.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EtcPasswdError> {
        if event.subject.trim().is_empty() {
            return Err(EtcPasswdError::EmptyEventSubject);
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

    fn sample_passwd(id: &str, name: &str) -> Passwd {
        Passwd {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EtcPasswd::new("EtcPasswd");
        assert_eq!(m.name, "EtcPasswd");
        assert_eq!(m.passwd_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_passwd() {
        let mut m = EtcPasswd::new("EtcPasswd");
        m.add_passwd(sample_passwd("a1", "Alpha")).unwrap();
        assert_eq!(m.passwd_count(), 1);
        assert!(m.get_passwd("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EtcPasswd::new("EtcPasswd");
        assert!(m.add_passwd(sample_passwd("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EtcPasswd::new("EtcPasswd");
        m.add_passwd(sample_passwd("x", "One")).unwrap();
        assert!(m.add_passwd(sample_passwd("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EtcPasswd::new("EtcPasswd");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
