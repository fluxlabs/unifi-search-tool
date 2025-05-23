
use constcat::concat;
use serde::{de::{self, Unexpected}, Deserialize};
use thiserror::Error;

pub mod validation;
use validation::MAC_ADDR_REGEX_STR;

/// Represents a 6-byte MAC address stored as the least significant 6 bytes of a u64.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MacAddress(u64);

impl MacAddress {
    /// Constructs a new `MacAddress` from a u64, asserting that it fits within 6 bytes.
    pub fn new(n: u64) -> MacAddress {
        assert!(
            n <= 0xFFFFFF_FFFFFF,
            "MAC Address value is larger than what fits in 6 bytes"
        );
        MacAddress(n)
    }

    /// Returns the MAC address as an 8-byte array (the first 2 bytes should be zero).
    #[inline]
    pub fn as_bytes(&self) -> [u8; 8] {
        let b = self.0.to_be_bytes();
        assert!(
            b[0] == 0 && b[1] == 0,
            "MAC Address value is larger than what fits in 6 bytes"
        );
        b
    }
}

impl From<u64> for MacAddress {
    #[inline]
    fn from(v: u64) -> Self {
        MacAddress::new(v)
    }
}

/// Represents an error that can occur while parsing a MAC address.
#[derive(Error, Debug)]
pub enum MacParseError {
    #[error("Invalid MAC Address: {invalid_mac:?}")]
    InvalidMac { invalid_mac: Box<str> },
}

impl std::str::FromStr for MacAddress {
    type Err = MacParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if !validation::text_is_valid_mac(input.as_bytes()) {
            return Err(MacParseError::InvalidMac {
                invalid_mac: Box::from(input),
            });
        }

        let mac_hex = input.replace([':', '-'], "");
        let num_u64 = u64::from_str_radix(&mac_hex, 16)
            .map_err(|_| MacParseError::InvalidMac { invalid_mac: input.into() })?;

        Ok(MacAddress::new(num_u64))
    }
}

impl std::convert::TryFrom<&'_ str> for MacAddress {
    type Error = MacParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::convert::TryFrom<std::borrow::Cow<'_, str>> for MacAddress {
    type Error = MacParseError;

    fn try_from(value: std::borrow::Cow<'_, str>) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bytes = self.as_bytes();
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
        )
    }
}

impl<'de> Deserialize<'de> for MacAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mac_str: &str = de::Deserialize::deserialize(deserializer)?;
        MacAddress::try_from(mac_str).map_err(|_| {
            let unexpected = Unexpected::Str(mac_str);
            const EXPECTED: &str =
                concat!("MAC Address in string format matching regex: ", MAC_ADDR_REGEX_STR);
            de::Error::invalid_value(unexpected, &EXPECTED)
        })
    }
}
