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
