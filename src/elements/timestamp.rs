use crate::utils::result::{EFValueResult, EFReturnEvent};
use chrono::{DateTime, Utc, FixedOffset, TimeDelta};

#[derive(Debug, Clone)]
pub enum EFUTCOffsetHemisphere {
    East,
    West
}

#[derive(Debug, Clone)]
pub struct EFUTCTimestamp(pub DateTime<Utc>);

impl EFUTCTimestamp {
    pub fn new_for_now() -> EFUTCTimestamp {
        EFUTCTimestamp(Utc::now())
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

    pub fn to_string(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_utc_offset(
        &self, 
        hemisphere: EFUTCOffsetHemisphere, 
        offset_hours: u32, offset_minutes: u32
    ) -> EFValueResult<DateTime<FixedOffset>> {
        let offset_seconds: i32 = (offset_hours * 3600 + offset_minutes * 60) as i32;
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

    pub fn apply_delta(&mut self, seconds: i64, nanoseconds: u32) -> EFValueResult<()> {
        let new_delta: TimeDelta = match TimeDelta::new(seconds, nanoseconds) {
            Some(d) => d,
            None => {
                return Err(EFReturnEvent::new_with_func_info_log(
                    "apply_delta", 
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
                    "apply_delta", 
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
    created: EFUTCTimestamp,
    last_accessed: EFUTCTimestamp,
    last_modified: EFUTCTimestamp
}

impl EFTimeMetadata {
    pub fn new() -> EFTimeMetadata {
        let new_timestamp: EFUTCTimestamp = EFUTCTimestamp::new_for_now();

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
                    "new_from_str", 
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

    pub fn get_created(&self) -> &EFUTCTimestamp {
        &self.created
    }
    pub fn get_last_accessed(&self) -> &EFUTCTimestamp {
        &self.last_accessed 
    }
    pub fn update_last_accessed(&mut self, new_timestamp: &EFUTCTimestamp) {
        self.last_accessed = new_timestamp.clone();
    }
    pub fn get_last_modified(&self) -> &EFUTCTimestamp {
        &self.last_modified
    }
    pub fn update_last_modified(&mut self, new_timestamp: &EFUTCTimestamp) {
        self.last_modified = new_timestamp.clone();
    }
}

#[derive(Debug, Clone)]
pub enum EFTimePeriod {
    Forever,
    Until(EFUTCTimestamp)
}
