use super::*;
use chrono::{DateTime, Utc, FixedOffset, TimeDelta};

#[derive(Debug, Clone)]
pub enum EFUTCOffsetHemisphere {
    East,
    West
}

#[derive(Debug, Clone)]
pub struct EFUTCTimestamp(pub DateTime<Utc>);

impl EFUTCTimestamp {
    pub fn new_now_timestamp() -> EFUTCTimestamp {
        EFUTCTimestamp(Utc::now())
    }

    pub fn from_str(s: &str) -> EFValueResult<EFUTCTimestamp> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(d) => Ok(EFUTCTimestamp(d.to_utc())),
            Err(_) => Err(EFReturnEvent::new_with_func_info_log(
                "EFUTCTimestamp::from_str", 
                "Expected RFC 3339 string slice and could not parse into UTC timestamp."
            ))
        }
    }

    pub fn to_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_utc_offset(
        &self, 
        hemisphere: EFUTCOffsetHemisphere, 
        offset_hours: i32, offset_minutes: i32, offset_seconds: i32
    ) -> EFValueResult<DateTime<FixedOffset>> {
        let total_offset_seconds: i32 = offset_hours * 3600 + offset_minutes * 60 + offset_seconds;
        let offset = match hemisphere {
            EFUTCOffsetHemisphere::East => match FixedOffset::east_opt(total_offset_seconds) {
                Some(o) => o,
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "EFUTCTimestamp::to_utc_offset", 
                        format!(
                            "Offset of {} seconds for east hemisphere not within range.", 
                            total_offset_seconds
                        ).as_str()
                    )); 
                }
            },
            EFUTCOffsetHemisphere::West => match FixedOffset::west_opt(total_offset_seconds) {
                Some(o) => o,
                None => {
                    return Err(EFReturnEvent::new_with_func_info_log(
                        "EFUTCTimestamp::to_utc_offset", 
                        format!(
                            "Offset of {} seconds for west hemisphere not within range.", 
                            total_offset_seconds
                        ).as_str()
                    )); 
                }
            }
        };

        Ok(self.0.with_timezone(&offset))
    }

    pub fn apply_delta(&mut self, seconds: i64, nanoseconds: u32) -> EFValueResult<()> {
        let new_delta: TimeDelta = match TimeDelta::new(seconds, nanoseconds) {
            Some(d) => d,
            None => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "EFUTCTimestamp::apply_delta", 
                    "Either the duration is out of bounds or nanoseconds is greater than or equal to one billion."
                ))
            }
        };

        self.0 = self.0 + new_delta;
        Ok(())
    }

    pub fn apply_delta_lazy(&mut self, days: i64, hours: i64, minutes: i64, seconds: i64) -> EFValueResult<()> {
        let total_seconds: i64 = days * 86400 + hours * 3600 + minutes * 60 + seconds;
        self.apply_delta(seconds, 0)
    }

    pub fn return_with_delta(&self, seconds: i64, nanoseconds: u32) -> EFValueResult<EFUTCTimestamp> {
        let new_delta: TimeDelta = match TimeDelta::new(seconds, nanoseconds) {
            Some(d) => d,
            None => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "EFUTCTimestamp::return_with_delta", 
                    "Either the duration is out of bounds or nanoseconds is greater than or equal to one billion."
                ))
            }
        };

        Ok(EFUTCTimestamp(self.0 + new_delta))
    }

    pub fn return_with_delta_lazy(&self, days: i64, hours: i64, minutes: i64, seconds: i64) -> EFValueResult<EFUTCTimestamp> {
        let total_seconds: i64 = days * 86400 + hours * 3600 + minutes * 60 + seconds;
        self.return_with_delta(seconds, 0)
    }
}

#[derive(Debug, Clone)]
pub struct EFTimeMetadata {
    pub created: EFUTCTimestamp,
    pub last_accessed: EFUTCTimestamp,
    pub last_modified: EFUTCTimestamp
}

impl EFTimeMetadata {
    pub fn new() -> EFTimeMetadata {
        let new_timestamp: EFUTCTimestamp = EFUTCTimestamp::new_now_timestamp();

        EFTimeMetadata {
            created: new_timestamp.clone(),
            last_accessed: new_timestamp.clone(),
            last_modified: new_timestamp
        }
    }

    pub fn new_from_str(s: &str) -> EFValueResult<EFTimeMetadata> {
        let new_timestamp: EFUTCTimestamp = match EFUTCTimestamp::from_str(s) {
            Ok(t) => t,
            Err(e) => {
                return Err(EFReturnEvent::with_added_func_info_log(
                    e, 
                    "EFTimeMetadata::new_from_str", 
                    "Could not make new time metadata object."
                ));
            }
        };

        Ok(EFTimeMetadata {
            created: new_timestamp.clone(),
            last_accessed: new_timestamp.clone(),
            last_modified: new_timestamp
        })
    }

    pub fn update_last_accessed(&mut self) {
        self.last_accessed = EFUTCTimestamp::new_now_timestamp();
    }

    pub fn update_last_modified(&mut self) {
        self.last_modified = EFUTCTimestamp::new_now_timestamp();
    }
}

#[derive(Debug, Clone)]
pub enum EFTimePeriod {
    Forever,
    Until(EFUTCTimestamp)
}
