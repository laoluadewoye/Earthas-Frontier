pub mod uri;
pub mod primitives;
pub mod common;
pub mod entity;
pub mod components;
pub mod timestamp;
pub mod byte_rep;
pub mod rule;
pub mod file;
pub mod tracker;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EFVersion(pub u8, pub u8, pub u8);

impl EFVersion {
    pub fn get_major(&self) -> u8 { self.0 }
    pub fn get_minor(&self) -> u8 { self.1 }
    pub fn get_patch(&self) -> u8 { self.2 }
}

#[derive(Debug, Clone)]
pub struct EFComponentTuple;
// {
//     version: EFVersion,
//     attrs: Vec<String>,
//     values: Vec<String>
// }


pub trait EFComponentRequest {}
pub trait EFComponentResponse {}

pub trait EFComponent: Clone {
    type ComponentParams;
    type ComponentRequestType: EFComponentRequest;
    type ComponentResponseType: EFComponentResponse;
    
    // Create functions
    fn create_new(params: Self::ComponentParams) -> Self;
    fn create_from_compatible(params: Self::ComponentParams, version: EFVersion) -> Self;
    fn create_from_older(older_component: &EFComponentTuple) -> Option<Self> where Self: Sized;

    // Getter functions
    fn get_component_as_older(&self, old_version: &EFVersion) -> EFComponentTuple;
    fn get_component_version(&self) -> &EFVersion;
    fn get_component_type(&self) -> &str;

    // Query functions
    fn handle_request(&self, request: &Self::ComponentRequestType) -> Self::ComponentResponseType;
}
