use super::*;
use super::formats::{EFId, EFName};
use super::uri::{EFURIString, EFURI};
use super::timestamp::{EFTimeMetadata, EFUTCTimestamp, EFTimePeriod};

pub trait EFCreateCompatible {
    type CreateParams;

    fn new_element(params: Self::CreateParams) -> Self;
    fn fallback_element(event: EFReturnEvent) -> Self;
    fn clone_element(&self) -> Self;
}

pub trait EFTupleCompatible {
    fn from_older_tuple(tuple: EFTuple) -> Self;
    fn to_older_tuple(&self) -> EFTuple;
}

pub trait EFMetadataCompatible {
    fn get_id(&self) -> &EFId;
    fn get_name(&self) -> &EFName;
    fn set_name(&mut self, new_string: &String) -> EFValueResult<()>;
    fn get_owner(&self) -> &EFURIString;
    fn set_owner(&mut self, new_owner: &EFURIString) -> EFValueResult<()>;
    fn get_system(&self) -> &EFURIString;
    fn get_uri(&self) -> &EFURI;
}

pub trait EFTimeMetadataCompatible {
    fn get_time_metadata(&self) -> &EFTimeMetadata;
    fn get_created(&self) -> &EFUTCTimestamp;
    fn get_last_accessed(&self) -> &EFUTCTimestamp;
    fn set_last_accessed(&mut self, new_timestamp: &EFUTCTimestamp) -> EFValueResult<()>;
    fn update_last_accessed(&mut self) -> EFValueResult<()>;
    fn get_last_modified(&self) -> &EFUTCTimestamp;
    fn set_last_modified(&mut self, new_timestamp: &EFUTCTimestamp) -> EFValueResult<()>;
    fn update_last_modified(&mut self) -> EFValueResult<()>;
}

pub trait EFValidityCompatible {
    fn get_validity_period(&self) -> &EFTimePeriod;
    fn set_validity_period(&mut self, new_period: &EFTimePeriod) -> EFValueResult<()>;
}

pub trait EFRetentionCompatible {
    fn get_retention_period(&self) -> &EFTimePeriod;
    fn set_retention_period(&mut self, new_period: &EFTimePeriod) -> EFValueResult<()>;
}

pub trait EFCacheCompatible {
    
}