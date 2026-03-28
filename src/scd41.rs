use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct SCD41 {
    sensor_id: u64,
    co2: u16,
    temperature: u16,
    humidity: u16,
}

impl SCD41 {
    pub fn new(sensor_id: u64, co2: u16, temperature: u16, humidity: u16) -> Self {
        Self {
            sensor_id,
            co2,
            temperature,
            humidity,
        }
    }

    pub fn try_from_raw(sensor_id: u64, raw: &[u8]) -> Result<Self, ()> {
        let co2 = parse_word([raw[0], raw[1], raw[2]])?;
        let temperature = parse_word([raw[3], raw[4], raw[5]])?;
        let humidity = parse_word([raw[6], raw[7], raw[8]])?;

        Ok(Self {
            sensor_id,
            co2,
            temperature,
            humidity,
        })
    }

    pub fn sensor_id(&self) -> u64 {
        self.sensor_id
    }

    pub fn sensor_id_display(&self) -> String {
        format!("{:x?}", self.sensor_id)
    }

    pub fn co2(&self) -> u16 {
        self.co2
    }

    pub fn temperature(&self) -> f32 {
        -45.0 + 175.0 * self.temperature as f32 / 65535.0
    }

    pub fn humidity(&self) -> f32 {
        100.0 * self.humidity as f32 / 65535.0
    }

    pub fn raw_co2(&self) -> u16 {
        self.co2
    }

    pub fn raw_temperature(&self) -> u16 {
        self.temperature
    }

    pub fn raw_humidity(&self) -> u16 {
        self.humidity
    }
}

impl Display for SCD41 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SCD41: CO2: {} ppm, temperature: {:.1}°C, relative humidity: {:.1}%",
            self.co2(), self.temperature(), self.humidity()
        )
    }
}

fn parse_word(bytes: [u8; 3]) -> Result<u16, ()> {
    let data = [bytes[0], bytes[1]];
    let actual = bytes[2];
    let expected = crc8(&data);

    if actual != expected {
        return Err(());
    }

    Ok(u16::from_be_bytes(data))
}

pub fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0xffu8;

    for byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = if (crc & 0x80) != 0 {
                (crc << 1) ^ 0x31
            } else {
                crc << 1
            };
        }
    }

    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_reading() -> SCD41 {
        let sensor_id = 0x1;
        let raw_reading = [0x01, 0xf4, 0x33, 0x66, 0x67, 0xa2, 0x5e, 0xb9, 0x3c];
        SCD41::try_from_raw(sensor_id, &raw_reading).unwrap()
    }

    #[test]
    fn test_display() {
        // Testing example from the official SCD41 datasheet
        let reading = SCD41 {
            sensor_id: 1,
            co2: 0x01f4u16,
            temperature: 0x6667u16,
            humidity: 0x5eb9u16,
        };

        let expected = "SCD41: CO2: 500 ppm, temperature: 25.0°C, relative humidity: 37.0%";
        let actual = format!("{}", reading);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_co2_conversion() {
        let reading = SCD41::new(1, 400, 2000, 5000);
        assert_eq!(reading.co2(), 400);

        // Testing example from the official SCD41 datasheet
        let reading = SCD41 {
            sensor_id: 1,
            co2: 0x01f4u16,
            temperature: 0x6667u16,
            humidity: 0x5eb9u16,
        };

        let expected = 500;
        let actual = reading.co2();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_temperature_conversion() {
        // Testing example from the official SCD41 datasheet
        let reading = SCD41 {
            sensor_id: 1,
            co2: 0x01f4u16,
            temperature: 0x6667u16,
            humidity: 0x5eb9u16,
        };

        let expected: f32 = 25.0;
        let actual = reading.temperature().floor();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_humidity_conversion() {
        // Testing example from the official SCD41 datasheet
        let reading = SCD41 {
            sensor_id: 1,
            co2: 0x01f4u16,
            temperature: 0x6667u16,
            humidity: 0x5eb9u16,
        };

        let expected: f32 = 37.0;
        let actual = reading.humidity().floor();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_crc8() {
        // Test data from the official SCD41 datasheet
        let data = 0xbeefu16.to_be_bytes();
        let expected = 0x92u8;
        assert_eq!(crc8(&data), expected);
    }

    #[test]
    fn test_reading_display() {
        let reading = SCD41::new(0u64, 0u16, 0u16, 0u16);
        let expected = "SCD41: CO2: 0 ppm, temperature: -45.0°C, relative humidity: 0.0%";
        assert_eq!(format!("{}", reading), format!("{}", expected));
    }

    #[test]
    fn test_try_from_raw() {
        let expected = SCD41 {
            sensor_id: 1,
            co2: 0x01f4u16,
            temperature: 0x6667u16,
            humidity: 0x5eb9u16,
        };

        let sensor_id = 0x01;
        let raw_reading = [0x01, 0xf4, 0x33, 0x66, 0x67, 0xa2, 0x5e, 0xb9, 0x3c];
        let actual = SCD41::try_from_raw(sensor_id, &raw_reading).unwrap();

        assert_eq!(expected, actual);
        assert_eq!(actual.sensor_id(), 1);
        assert_eq!(actual.co2(), 500);
        assert_eq!(actual.temperature().floor(), 25.0);
        assert_eq!(actual.humidity().floor(), 37.0);
    }

    #[test]
    fn test_sensor_id_display() {
        let sensor_id = 0x1;
        let raw_reading = [0x01, 0xf4, 0x33, 0x66, 0x67, 0xa2, 0x5e, 0xb9, 0x3c];
        let actual = SCD41::try_from_raw(sensor_id, &raw_reading).unwrap();

        assert_eq!(actual.sensor_id_display(), "1");
    }

    #[test]
    fn test_raw_co2() {
        let reading = get_reading();
        assert_eq!(reading.raw_co2(), 0x01f4u16);
    }

    #[test]
    fn test_raw_temperature() {
        let reading = get_reading();
        assert_eq!(reading.raw_temperature(), 0x6667u16);
    }

    #[test]
    fn test_raw_humidity() {
        let reading = get_reading();
        assert_eq!(reading.raw_humidity(), 0x5eb9u16);
    }

    #[test]
    fn test_parse_word_working() {
        let raw_reading = [0x01, 0xf4, 0x33];
        let expected = parse_word(raw_reading).unwrap();
        let actual = 500u16;
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_parse_word_failing() {
        let raw_reading = [0x01, 0xf4, 0x34];
        let _ = parse_word(raw_reading).unwrap();
    }
}