use super::*;
use std::fmt::Display;

pub trait EFFormattedString {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized;
}

#[derive(Debug, Clone)]
pub struct EFName(pub String);

impl EFFormattedString for EFName {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized {
        let is_valid: bool = new_string.chars().all(|c: char| {
            c.is_alphanumeric() || c == '_' || c == '-' || c == ' '
        });

        if is_valid {
            Ok(EFName(new_string.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "EFName::new_if_valid", 
                "String is not a valid name."
            ))
        }
    }
}

impl Display for EFName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct EFId(String);

impl EFFormattedString for EFId {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized {
        let is_valid: bool = new_string.chars().all(|c: char| {
            c.is_ascii_hexdigit()
        });

        if is_valid {
            Ok(EFId(new_string.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "EFId::new_if_valid", 
                "String is not a valid id."
            ))
        }
    }
}

impl Display for EFId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct EFDomainName(String);

impl EFFormattedString for EFDomainName {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized {
        let is_valid: bool = new_string.chars().all(|c: char| {
            c.is_alphanumeric() || c == '.'
        });

        if is_valid {
            Ok(EFDomainName(new_string.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "EFDomainName::new_if_valid", 
                "String is not a valid domain name."
            ))
        }
    }
}

impl Display for EFDomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct EFElementName(pub String);

impl EFFormattedString for EFElementName {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized {
        let is_valid: bool = new_string.chars().all(|c: char| {
            c.is_alphabetic() || c.is_lowercase() || c == '_'
        });

        if is_valid {
            Ok(EFElementName(new_string.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "EFElementName::new_if_valid", 
                "String is not a valid element name."
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct EFRequestName(pub String);

impl EFFormattedString for EFRequestName {
    fn new_if_valid(new_string: &String) -> EFValueResult<Self> where Self: Sized {
        let is_valid: bool = new_string.chars().all(|c: char| {
            c.is_alphabetic() || c.is_lowercase() || c == '_'
        });

        if is_valid {
            Ok(EFRequestName(new_string.clone()))
        }
        else {
            Err(EFReturnEvent::new_with_func_info_log(
                "EFRequestName::new_if_valid", 
                "String is not a valid request name."
            ))
        }
    }
}
