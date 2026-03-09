use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    value: String,
}

impl Location {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let _location = Location::new("test".into());
    }

    #[test]
    fn test_get_value() {
        let location = Location::new("test".into());
        assert_eq!(location.value(), "test");
    }
}