use std::net::{IpAddr};
use strum_macros::Display as Strum_Display;
use super::formats::{EFDomainName, EFName, EFId, EFElementName, EFRequestName};

#[derive(Strum_Display)]
pub enum EFURIConnection {
    #[strum(to_string = "Ip:{0}")]
    Ip(IpAddr),

    #[strum(to_string = "Domain:{0}")]
    Domain(EFDomainName)
}

#[derive(Strum_Display)]
pub enum EFURIAuthority {
    Global,
    Local,

    #[strum(to_string = "Connection:{0}")]
    Connection(EFURIConnection)
}

pub struct EFURIPath(EFName);

#[derive(Strum_Display)]
pub enum EFURIEntity {
    #[strum(to_string = "Id:{0}")]
    Id(EFId),

    #[strum(to_string = "Name:{0}")]
    Name(EFName)
}

pub struct EFURIString(String);

pub struct EFURI {
    authority: EFURIAuthority,
    paths: Vec<EFURIPath>,
    element: EFElementName,
    entity: EFURIEntity,
    request: EFRequestName
}

impl EFURI {
    pub fn to_uri_string(&self) -> EFURIString {
        let mut new_uri_string: String = String::from("$");

        // Add authority
        new_uri_string.push_str(&self.authority.to_string().as_str());
        new_uri_string.push('/');

        // Add paths
        for path in self.paths.iter() {
            new_uri_string.push_str(path.0.0.as_str());
            new_uri_string.push('/');
        }

        // Add element
        new_uri_string.push('#');
        new_uri_string.push_str(self.element.0.as_str());
        new_uri_string.push('/');

        // Add entity specifier
        new_uri_string.push_str(&self.entity.to_string().as_str());
        new_uri_string.push('?');

        // Add request
        new_uri_string.push_str(&self.request.0.as_str());

        // Return URI string
        EFURIString(new_uri_string)
    }
}

pub struct EFRequest {

}

pub struct EFResponse {

}
