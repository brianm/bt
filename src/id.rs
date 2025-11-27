use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Crockford base32 alphabet (lowercase, excludes i, l, o, u)
const CROCKFORD: &[u8] = b"0123456789abcdefghjkmnpqrstvwxyz";

/// A task ID using Crockford base32 encoding (e.g., a1b2c3d4)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(String);

impl TaskId {
    /// Generate a new unique task ID (40 bits of randomness, 8 base32 chars)
    pub fn new() -> Self {
        let random_bytes: [u8; 5] = rand::thread_rng().gen(); // 40 bits
        let mut id = String::with_capacity(8);

        // Encode 40 bits as 8 base32 chars (5 bits each)
        let bits = u64::from_be_bytes([
            0,
            0,
            0,
            random_bytes[0],
            random_bytes[1],
            random_bytes[2],
            random_bytes[3],
            random_bytes[4],
        ]);

        for i in (0..8).rev() {
            let idx = ((bits >> (i * 5)) & 0x1f) as usize;
            id.push(CROCKFORD[idx] as char);
        }

        TaskId(id)
    }

    /// Create a TaskId from an existing string (for parsing)
    pub fn from_string(s: impl Into<String>) -> Self {
        TaskId(s.into())
    }

    /// Get the full ID string
    #[allow(dead_code)]
    pub fn full(&self) -> &str {
        &self.0
    }

    /// Get the shortest prefix that uniquely identifies this ID among others
    pub fn shortest_unique_prefix<'a>(&'a self, others: &[&TaskId]) -> &'a str {
        for len in 1..=self.0.len() {
            let prefix = &self.0[..len];
            let is_unique = others.iter().all(|other| {
                other.0 == self.0
                    || !other
                        .0
                        .to_lowercase()
                        .starts_with(&prefix.to_lowercase())
            });
            if is_unique {
                return prefix;
            }
        }
        &self.0
    }

    /// Check if this ID matches a prefix (case-insensitive)
    pub fn matches_prefix(&self, prefix: &str) -> bool {
        self.0.to_lowercase().starts_with(&prefix.to_lowercase())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for TaskId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id_format() {
        let id = TaskId::new();
        assert_eq!(id.0.len(), 8); // 8 base32 chars
        // All chars should be valid Crockford base32
        for c in id.0.chars() {
            assert!(
                CROCKFORD.contains(&(c as u8)),
                "Invalid char '{}' in ID",
                c
            );
        }
    }

    #[test]
    fn test_unique_ids() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();
        assert_ne!(id1, id2); // Should be different due to randomness
    }

    #[test]
    fn test_prefix_matching_case_insensitive() {
        let id = TaskId::from_string("a1b2c3d4");
        assert!(id.matches_prefix("a1b2"));
        assert!(id.matches_prefix("A1B2")); // case-insensitive
        assert!(id.matches_prefix("a1"));
        assert!(id.matches_prefix("a"));
        assert!(!id.matches_prefix("b2"));
    }

    #[test]
    fn test_shortest_unique_prefix() {
        let id1 = TaskId::from_string("a1b2c3d4");
        let id2 = TaskId::from_string("a1b2xxxx");
        let id3 = TaskId::from_string("xxxxxxxx");

        // id1 vs id2: need "a1b2c" to distinguish
        let others: Vec<&TaskId> = vec![&id2, &id3];
        assert_eq!(id1.shortest_unique_prefix(&others), "a1b2c");

        // id3 vs id1, id2: just "x" is enough
        let others: Vec<&TaskId> = vec![&id1, &id2];
        assert_eq!(id3.shortest_unique_prefix(&others), "x");

        // Single ID: just 1 char needed
        let others: Vec<&TaskId> = vec![];
        assert_eq!(id1.shortest_unique_prefix(&others), "a");
    }
}
