use crate::utils::result::{EFOk, EFError};
use crate::utils::byte_vector::get_string_from_byte_vector;
use chrono::{DateTime, Utc, FixedOffset};

#[derive(Debug, Clone)]
pub struct EFVersion(pub u8, pub u8, pub u8);

impl EFVersion {
    pub fn get_major(&self) -> u8 { self.0 }
    pub fn get_minor(&self) -> u8 { self.1 }
    pub fn get_patch(&self) -> u8 { self.2 }
}

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

pub trait EFComponent {
    type ComponentParams;

    fn new(params: Self::ComponentParams) -> Self;
    fn build(params: Self::ComponentParams, version: EFVersion) -> Self;
    fn get_component_str(&self) -> String;
    fn get_component_version(&self) -> EFVersion;
    fn clone_component(&self) -> Self;
    fn handle_request(&self, request: &efid::EFQuery) -> efid::EFResponse;
}

// Note: implementations should prefer big endian byte order
#[derive(Debug)]
pub struct EFByteRep {
    pub bytes: Vec<u8>
}

impl EFComponent for EFByteRep {
    type ComponentParams = Vec<u8>;

    fn new(params: Self::ComponentParams) -> Self {
        EFByteRep { bytes: params }
    }

    fn build(params: Self::ComponentParams, _version: EFVersion) -> Self {
        EFByteRep { bytes: params }
    }

    fn get_component_str(&self) -> String {
        // Get component start and start of first attribute
        let (component_start, attr_one_start) = match self.bytes.get(2..4) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return String::from(""); }
        };

        // Return the component string
        match self.bytes.get(component_start..attr_one_start) {
            Some(b) => match String::from_utf8(b.to_vec()) {
                Ok(s) => s,
                Err(_) => String::from("")
            },
            None => String::from("")
        }
    }

    fn get_component_version(&self) -> EFVersion {
        // Get version start and component start
        let (version_start, component_start) = match self.bytes.get(1..3) {
            Some(b) => (b[0] as usize, b[1] as usize),
            None => { return EFVersion(0u8, 0u8, 0u8); }
        };

        // Return the version numbers
        match self.bytes.get(version_start..component_start) {
            Some(b) => EFVersion(b[0], b[1], b[2]),
            None => return EFVersion(0u8, 0u8, 0u8)
        }
    }

    fn clone_component(&self) -> Self {
        EFByteRep { bytes: self.bytes.clone() }
    }

    fn handle_request(&self, request: &efid::EFQuery) -> efid::EFResponse {
        efid::EFResponse
    }
}

pub struct EFByteRepBuilder {
    pub byte_vectors: Vec<Vec<u8>>,
    pub version_vector: Vec<u8>,
    pub component_vector: Vec<u8>
}

pub trait EFByteRepCompatible {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError>;
    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub trait EFByteRepCompatibleEnum {
    fn get_byte_vec(&self) -> Vec<u8>;
    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized;
}

pub mod efid;
pub mod primitives;
pub mod common;
pub mod entity;
pub mod components;
