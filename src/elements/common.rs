use crate::elements::{EFByteRep, EFComponent, EFByteRepCompatible};
use crate::utils::result::{EFOk, EFError};

impl EFComponent for String {
    type ComponentParams = String;

    fn get_component_str(&self) -> String {
        String::from("string")
    }
}

impl EFByteRepCompatible for String {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        Ok(EFOk{
            value: EFByteRep { bytes: self.clone().into_bytes(), component: String::from("string") },
            msg: String::from("Created byte rep from string.")
        })
    }

    fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        if !br.component.eq("string") {
            return Err(EFError{
                function: String::from("from_byte_rep"), 
                line: String::from("!br.component.eq(\"string\")"), 
                msg: String::from("Component is not set to string.")
            });
        }

        match String::from_utf8(br.bytes.clone()) {
            Ok(s) => Ok(EFOk{ 
                value: s,
                msg: String::from("Converted the byte rep into a string.")
            }),
            Err(_) => Err(EFError{
                function: String::from("from_byte_rep"), 
                line: String::from("String::from_utf8(br.bytes.clone())"), 
                msg: String::from("Passed in byte rep is not compatible with UTF-8.")
            })
        }
    }
}
