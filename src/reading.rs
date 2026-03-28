use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::bme280::BME280;
use crate::ds18b20::DS18B20;
use crate::scd41::SCD41;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Reading {
    BME280(BME280),
    DS18B20(DS18B20),
    SCD41(SCD41),
}

impl Display for Reading {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reading::BME280(bme280) => bme280.fmt(f),
            Reading::DS18B20(ds18b20) => ds18b20.fmt(f),
            Reading::SCD41(scd41) => scd41.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_display() {
        let bme280_reading = Reading::BME280(BME280::new(25.0, 100000.0, 50.0));
        let ds18b20_reading = Reading::DS18B20(DS18B20::new("28-000000000000".to_string(), 25000));
        let scd41_reading = Reading::SCD41(SCD41::new(0u64, 0u16, 0u16, 0u16));

        assert_eq!(format!("{}", bme280_reading), format!("{}", bme280_reading));
        assert_eq!(format!("{}", ds18b20_reading), format!("{}", ds18b20_reading));
        assert_eq!(format!("{}", scd41_reading), format!("{}", scd41_reading));
    }
}