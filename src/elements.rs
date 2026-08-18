use crate::utils::result::{EFOk, EFError};

#[derive(Debug, Clone)]
pub struct EFVersion(pub u8, pub u8, pub u8);

impl EFVersion {
    pub fn get_major(&self) -> u8 { self.0 }
    pub fn get_minor(&self) -> u8 { self.1 }
    pub fn get_patch(&self) -> u8 { self.2 }
}

pub struct EFComponentTuple;

pub trait EFComponent {
    type ComponentParams;

    fn new(params: Self::ComponentParams) -> Self;
    fn build(params: Self::ComponentParams, version: EFVersion) -> Self;
    fn get_component_str(&self) -> String;
    fn get_component_version(&self) -> EFVersion;
    fn clone_component(&self) -> Self;
    fn upgrade_component(older_componet: EFComponentTuple);
    fn downgrade_component(&self);
    fn handle_request(&self, request: &uri::EFQuery) -> uri::EFResponse;
}

pub trait EFDynamicComponent {
    type ComponentParams;

    fn new(params: Self::ComponentParams) -> Self;
}

pub mod uri;
pub mod primitives;
pub mod common;
pub mod entity;
pub mod components;
pub mod timestamp;
pub mod byte_rep;
pub mod rule;
