// Heavy use statements to bring functionality into scope
use crate::elements::{EFByteRep, EFByteRepBuilder, EFComponent, EFByteRepCompatible, EFVersion};
use crate::elements::efid::{EFQuery, EFResponse};
use crate::utils::result::{EFOk, EFError};
use crate::utils::versions::*;
use crate::utils::byte_vector::{
    get_string_from_byte_vector,
    get_byte_rep_from_builder, 
    get_builder_from_byte_rep
};
use crate::utils::generic_vector::get_index_from_generic_vector;

pub mod unsigned_int {
    use super::*;

    #[derive(Debug)]
    pub struct EFUSize(pub usize, pub EFVersion);

    impl EFComponent for EFUSize {
        type ComponentParams = usize;

        fn new(params: Self::ComponentParams) -> Self {
            EFUSize(params, EFUSIZE_VERSION)
        }

        fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
            EFUSize(params, version)
        }

        fn get_component_str(&self) -> String {
            String::from("usize")
        }

        fn get_component_version(&self) -> EFVersion {
            self.1.clone()
        }

        fn clone_component(&self) -> Self {
            EFUSize(self.0, self.1.clone())
        }

        fn handle_request(&self, request: &EFQuery) -> EFResponse {
            EFResponse
        }
    }

    impl EFByteRepCompatible for EFUSize {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            // Create vectors from metadata
            let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
            let component_vector: Vec<u8> = self.get_component_str().into_bytes();

            // Create vectors from attributes
            let usize_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

            // Return byte rep
            let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
                byte_vectors: vec![usize_bytes], version_vector, component_vector
            };
            get_byte_rep_from_builder(&mut builder)
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            // Get the vectors for each attribute
            let builder: EFByteRepBuilder = match get_builder_from_byte_rep(br) {
                Ok(b) => b.value,
                Err(e) => { return Err(e); }
            };

            // Check the component
            let component: String = match get_string_from_byte_vector(builder.component_vector.clone()) {
                Ok(s) => s.value,
                Err(e) => { return Err(e); }
            };
            if !component.eq("usize") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!component.eq(\"usize\")"), 
                    msg: String::from("Component is not set to usize.")
                });
            }

            // Create the version
            let version: EFVersion = match builder.version_vector.len() == 3 {
                true => EFVersion(
                    builder.version_vector[0], 
                    builder.version_vector[1], 
                    builder.version_vector[2]
                ),
                false => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("builder.version_vector.len() == 3"), 
                        msg: String::from("Could not parse version for usize.")
                    });
                }
            };

            // Create the value
            match get_index_from_generic_vector(&builder.byte_vectors, 0) {
                Ok(b_vec) => {
                    let b_arr: [u8; 8] = match b_vec.value.try_into() {
                        Ok(b) => b,
                        Err(_) => {
                            return Err(EFError{
                                function: String::from("from_byte_rep"), 
                                line: String::from("b_vec.try_into()"), 
                                msg: String::from("Could not convert the byte rep into a 8-byte array for usize.")
                            });
                        }
                    };

                    Ok(EFOk{ 
                        value: EFUSize(usize::from_be_bytes(b_arr), version), 
                        msg: String::from("Converted the byte rep into a usize.") 
                    })
                },
                Err(e) => { return Err(e); }
            }
        }
    }

    #[derive(Debug)]
    pub struct EFU8(pub u8);

    impl EFComponent for EFU8 {
        fn get_component_str(&self) -> String {
            String::from("u8")
        }
    }

    impl EFByteRepCompatible for EFU8 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: vec![self.0], component: String::from("u8") },
                msg: String::from("Created byte rep from u8.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("u8") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"u8\")"), 
                    msg: String::from("Component is not set to u8.")
                });
            }

            let g_arr: [u8; 1] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 1-byte array for u8.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFU8(g_arr[0]),
                msg: String::from("Converted the byte rep into a u8.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFU16(pub u16);

    impl EFComponent for EFU16 {
        fn get_component_str(&self) -> String {
            String::from("u16")
        }
    }

    impl EFByteRepCompatible for EFU16 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("u16") },
                msg: String::from("Created byte rep from u16.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("u16") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"u16\")"), 
                    msg: String::from("Component is not set to u16.")
                });
            }

            let g_arr: [u8; 2] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 2-byte array for u16.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFU16(u16::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a u16.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFU32(pub u32);

    impl EFComponent for EFU32 {
        fn get_component_str(&self) -> String {
            String::from("u32")
        }
    }

    impl EFByteRepCompatible for EFU32 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("u32") },
                msg: String::from("Created byte rep from u32.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("u32") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"u32\")"), 
                    msg: String::from("Component is not set to u32.")
                });
            }

            let g_arr: [u8; 4] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 4-byte array for u32.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFU32(u32::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a u32.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFU64(pub u64);

    impl EFComponent for EFU64 {
        fn get_component_str(&self) -> String {
            String::from("u64")
        }
    }
    
    impl EFByteRepCompatible for EFU64 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("u64") },
                msg: String::from("Created byte rep from u64.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("u64") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"u64\")"), 
                    msg: String::from("Component is not set to u64.")
                });
            }

            let g_arr: [u8; 8] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 8-byte array for u64.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFU64(u64::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a u64.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFU128(pub u128);

    impl EFComponent for EFU128 {
        fn get_component_str(&self) -> String {
            String::from("u128")
        }
    }
    
    impl EFByteRepCompatible for EFU128 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("u128") },
                msg: String::from("Created byte rep from u128.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("u128") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"u128\")"), 
                    msg: String::from("Component is not set to u128.")
                });
            }

            let g_arr: [u8; 16] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 16-byte array for u128.")
                    });
                }
            };

            Ok(EFOk{
                value: EFU128(u128::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a u128.") 
            })
        }
    }
}