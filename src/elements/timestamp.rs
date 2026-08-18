use crate::utils::result::{EFOk, EFError};
use crate::utils::byte_vector::get_string_from_byte_vector;
use chrono::{DateTime, Utc, FixedOffset};

#[derive(Debug, Clone)]
pub enum EFUTCOffsetHemisphere {
    East,
    West
}

#[derive(Debug, Clone)]
pub struct EFUTCTimestamp(pub DateTime<Utc>);

impl EFUTCTimestamp {
    pub fn get_timestamp_for_now() -> Self {
        EFUTCTimestamp(Utc::now())
    }

    pub fn to_utc_offset(
        &self, 
        hemisphere: EFUTCOffsetHemisphere, 
        offset_hours: i32, offset_minutes: i32
    ) -> Result<EFOk<DateTime<FixedOffset>>, EFError> {
        let offset_seconds: i32 = offset_hours * 3600 + offset_minutes * 60;
        let (offset, hemisphere_str) = match hemisphere {
            EFUTCOffsetHemisphere::East => match FixedOffset::east_opt(offset_seconds) {
                Some(fo) => (fo, "east"),
                None => {
                    return Err(EFError{
                        function: String::from("to_utc_offset"),
                        line: String::from("FixedOffset::east_opt(offset_seconds)"),
                        msg: format!("Offset of {} seconds for eastern hemisphere not within range.", offset_seconds)
                    });
                }
            },
            EFUTCOffsetHemisphere::West => match FixedOffset::west_opt(offset_seconds) {
                Some(fo) => (fo, "west"),
                None => {
                    return Err(EFError{
                        function: String::from("to_utc_offset"),
                        line: String::from("FixedOffset::west_opt(offset_seconds)"),
                        msg: format!("Offset of {} seconds for wester hemisphere not within range.", offset_seconds)
                    });
                }
            }
        };

        Ok(EFOk{
            value: self.0.with_timezone(&offset), 
            msg: format!(
                "Created datetime with an offset of {} hours and {} minutes {}.", 
                offset_hours, offset_minutes, hemisphere_str
            )
        })
    }

    pub fn to_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_byte_vector(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn from_string(s: &str) -> Result<EFOk<Self>, EFError> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(d) => Ok(EFOk{
                value: EFUTCTimestamp(d.to_utc()), 
                msg: String::from("Parsed RFC 3339 string into UTC timestamp.")
            }),
            Err(_) => Err(EFError{
                function: String::from("from_string"),
                line: String::from("DateTime::parse_from_rfc3339(s)"),
                msg: String::from("Could not parse RFC 3339 string into UTC timestamp.")
            })
        }
    }

    pub fn from_byte_vector(byte_vector: Vec<u8>) -> Result<EFOk<Self>, EFError> {
        match get_string_from_byte_vector(byte_vector) {
            Ok(s) => match EFUTCTimestamp::from_string(s.value.as_str()) {
                Ok(ts) => Ok(EFOk{
                    value: ts.value, 
                    msg: String::from("Parsed byte vector into UTC timestamp.")
                }),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}
