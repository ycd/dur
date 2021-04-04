use std::{
    collections::HashMap,
    error::Error,
    net::IpAddr,
    time::{Duration, SystemTime},
};

use crate::Backend;
// In memory baceknd for dur
#[derive(Debug, Clone)]
pub struct Memory {
    record: HashMap<u64, HashMap<Duration, Option<IpAddr>>>,
}

impl Backend for Memory {
    fn new() -> Self {
        Self {
            record: HashMap::new(),
        }
    }

    // inserts the incoming request to the
    fn insert(
        &mut self,
        id: u64,
        ip_addr: Option<std::net::IpAddr>,
    ) -> Result<usize, Box<dyn Error>> {
        let key = self.record.entry(id).or_insert(HashMap::new());
        key.insert(
            SystemTime::now().duration_since(std::time::UNIX_EPOCH)?,
            ip_addr,
        );

        Ok(key.len())
    }

    fn clear(&mut self) {
        self.record.clear();
    }

    fn len(&self) -> usize {
        self.record.len()
    }

    // Get the current request count of the id
    fn request_count(&self, id: u64) -> usize {
        match self.record.get(&id) {
            None => 0,
            Some(v) => v.len(),
        }
    }

    fn evict_older_timestamps(&mut self, id: u64, timestamp: Duration, window_time: u16) {
        match self.record.get_mut(&id) {
            Some(logs) => {
                for (duration, _) in logs.clone().iter() {
                    if (timestamp.as_secs() - duration.as_secs()) > window_time as u64 {
                        logs.remove(duration);
                    }
                }
            }
            None => return,
        }
    }
}

impl Memory {
    // Get the count of unique ip addresses for the user
    fn unique_ip_addresses(&self, id: u64) -> usize {
        match self.record.get(&id) {
            Some(v) => v.iter().filter(|(_, &ip_addr)| ip_addr.is_some()).count(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_insert() {
        let mut mem = Memory::new();

        assert!(mem.insert(1234859, None).is_ok());
    }
    #[test]
    fn test_insert_multiple() {
        let mut mem = Memory::new();

        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
    }

    #[test]
    fn test_len_and_cleanup() {
        let mut mem = Memory::new();

        assert_eq!(mem.len(), 0);

        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());

        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());

        mem.clear();
        assert_eq!(mem.len(), 0);
    }

    #[test]
    fn test_request_count() {
        let mut mem = Memory::new();

        assert_eq!(mem.len(), 0);

        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());
        assert!(mem.insert(12348591, None).is_ok());

        assert!(mem.insert(12384, None).is_ok());
        assert!(mem.insert(12384, None).is_ok());
        assert!(mem.insert(12384, None).is_ok());

        assert_eq!(mem.request_count(12348591), 5);
        assert_eq!(mem.request_count(12384), 3);

        mem.clear();
        assert_eq!(mem.len(), 0);

        assert_eq!(mem.request_count(12348591), 0);
        assert_eq!(mem.request_count(12384), 0);
    }

    #[test]
    fn test_unique_ip_addresses() {
        let mut mem = Memory::new();

        assert_eq!(mem.len(), 0);

        assert!(mem
            .insert(12348591, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .is_ok());
        assert!(mem
            .insert(12348591, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .is_ok());
        assert!(mem
            .insert(12348591, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .is_ok());
        assert!(mem
            .insert(12348591, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .is_ok());
        assert!(mem
            .insert(12348591, Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .is_ok());

        assert_eq!(mem.unique_ip_addresses(12348591), 5);
    }
}
