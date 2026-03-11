use crate::reading::Reading;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;
use crate::location::Location;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Record {
    id: Uuid,
    timestamp: DateTime<Utc>,
    location: Location,
    reading: Reading,
}

impl Record {
    pub fn new(id: Uuid, timestamp: DateTime<Utc>, location: Location,reading: Reading) -> Self {
        Self {
            id,
            timestamp,
            location,
            reading,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn reading(&self) -> &Reading {
        &self.reading
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        let timestamp = self.timestamp();
        let reading_display = format!("{}", self.reading);
        write!(f, "[{}] {} ({})", timestamp, reading_display, id)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp().cmp(&other.timestamp()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bme280::BME280;

    #[test]
    fn test_record() {
        let _record = Record {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            location: Location::new("some location".into()),
            reading: Reading::BME280(BME280::new(22.625, 101325.0, 35.0)),
        };
    }

    #[test]
    fn test_new() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let location = Location::new("some location".into());
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let reading_copy = reading.clone();
        let record = Record::new(id, timestamp, location, reading);
        assert_eq!(record.id(), id);
        assert_eq!(record.timestamp(), timestamp);
        assert_eq!(record.reading(), &reading_copy);
    }

    #[test]
    fn test_id() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let location = Location::new("some location".into());
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, location, reading);
        assert_eq!(record.id(), id);
    }

    #[test]
    fn test_timestamp() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let location = Location::new("some location".into());
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, location, reading);
        assert_eq!(record.timestamp(), timestamp);
    }

    #[test]
    fn test_reading() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let location = Location::new("some location".into());
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let reading_copy = reading.clone();
        let record = Record::new(id, timestamp, location, reading);
        assert_eq!(record.reading(), &reading_copy);
    }

    #[test]
    fn test_display() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let location = Location::new("some location".into());
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let record = Record::new(id, timestamp, location, reading);

        let expected = format!(
            "[{}] BME280: 22.62 °C, 101325.00 Pa, 35.00% ({})",
            timestamp, id
        );
        let actual = format!("{}", record);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_reading() {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();
        let reading = Reading::BME280(BME280::new(22.625, 101325.0, 35.0));
        let location = Location::new("some location".into());
        let record = Record::new(id, timestamp, location, reading);
        assert_eq!(record.location.value(), "some location");
    }
}
