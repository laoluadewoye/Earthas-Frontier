use crate::utils::result::{EFValueResult, EFReturnEvent};
use chrono::{DateTime, Utc, FixedOffset};

#[derive(Debug, Clone)]
pub enum EFUTCOffsetHemisphere {
    East,
    West
}

#[derive(Debug, Clone)]
pub struct EFUTCTimestamp(pub DateTime<Utc>);

impl EFUTCTimestamp {
    pub fn new_for_now() -> Self {
        EFUTCTimestamp(Utc::now())
    }

    pub fn to_utc_offset(
        &self, 
        hemisphere: EFUTCOffsetHemisphere, 
        offset_hours: i32, offset_minutes: i32
    ) -> EFValueResult<DateTime<FixedOffset>> {
        let offset_seconds: i32 = offset_hours * 3600 + offset_minutes * 60;
        let (offset, hemisphere_str) = match hemisphere {
            EFUTCOffsetHemisphere::East => match FixedOffset::east_opt(offset_seconds) {
                Some(fo) => (fo, "east"),
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "to_utc_offset", 
                        format!("Offset of {} seconds for eastern hemisphere not within range.", offset_seconds).as_str()
                    ));
                }
            },
            EFUTCOffsetHemisphere::West => match FixedOffset::west_opt(offset_seconds) {
                Some(fo) => (fo, "west"),
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "to_utc_offset", 
                        format!("Offset of {} seconds for western hemisphere not within range.", offset_seconds).as_str()
                    ));
                }
            }
        };

        Ok(self.0.with_timezone(&offset))
    }

    pub fn to_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_byte_vector(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn from_str(s: &str) -> EFValueResult<EFUTCTimestamp> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(d) => Ok(EFUTCTimestamp(d.to_utc())),
            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                "from_str", 
                "Could not parse RFC 3339 string into UTC timestamp."
            ))
        }
    }
}
