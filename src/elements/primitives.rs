// Heavy use statements to bring functionality into scope
use crate::elements::{EFByteRep, EFComponent, EFByteRepCompatible};
use crate::utils::result::{EFOk, EFError};

pub mod unsigned_int {
    use super::{EFByteRep, EFComponent, EFByteRepCompatible};
    use super::{EFOk, EFError};

    #[derive(Debug)]
    pub struct EFUSize(pub usize);

    impl EFComponent for EFUSize {
        fn get_component_str(&self) -> String {
            String::from("usize")
        }
    }

    impl EFByteRepCompatible for EFUSize {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("usize") },
                msg: String::from("Created byte rep from usize.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("usize") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"usize\")"), 
                    msg: String::from("Component is not set to usize.")
                });
            }

            let g_arr: [u8; 8] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 8-byte array for usize.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFUSize(usize::from_be_bytes(g_arr)), 
                msg: String::from("Converted the byte rep into a usize.") 
            })
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

pub mod signed_int {
    use super::{EFByteRep, EFComponent, EFByteRepCompatible};
    use super::{EFOk, EFError};

    #[derive(Debug)]
    pub struct EFISize(pub isize);

    impl EFComponent for EFISize {
        fn get_component_str(&self) -> String {
            String::from("isize")
        }
    }
    
    impl EFByteRepCompatible for EFISize {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("isize") },
                msg: String::from("Created byte rep from isize.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("isize") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"isize\")"), 
                    msg: String::from("Component is not set to isize.")
                });
            }

            let g_arr: [u8; 8] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 8-byte array for isize.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFISize(isize::from_be_bytes(g_arr)), 
                msg: String::from("Converted the byte rep into a isize.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFI8(pub i8);

    impl EFComponent for EFI8 {
        fn get_component_str(&self) -> String {
            String::from("i8")
        }
    }
    
    impl EFByteRepCompatible for EFI8 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: vec![self.0.cast_unsigned()], component: String::from("i8") },
                msg: String::from("Created byte rep from i8.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("i8") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"i8\")"), 
                    msg: String::from("Component is not set to i8.")
                });
            }

            let g_arr: [u8; 1] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 1-byte array for i8.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFI8(g_arr[0].cast_signed()),
                msg: String::from("Converted the byte rep into a i8.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFI16(pub i16);

    impl EFComponent for EFI16 {
        fn get_component_str(&self) -> String {
            String::from("i16")
        }
    }
    
    impl EFByteRepCompatible for EFI16 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("i16") },
                msg: String::from("Created byte rep from i16.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("i16") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"i16\")"), 
                    msg: String::from("Component is not set to i16.")
                });
            }

            let g_arr: [u8; 2] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 2-byte array for i16.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFI16(i16::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a i16.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFI32(pub i32);

    impl EFComponent for EFI32 {
        fn get_component_str(&self) -> String {
            String::from("i32")
        }
    }
    
    impl EFByteRepCompatible for EFI32 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("i32") },
                msg: String::from("Created byte rep from i32.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("i32") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"i32\")"), 
                    msg: String::from("Component is not set to i32.")
                });
            }

            let g_arr: [u8; 4] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 4-byte array for i32.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFI32(i32::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a i32.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFI64(pub i64);

    impl EFComponent for EFI64 {
        fn get_component_str(&self) -> String {
            String::from("i64")
        }
    }
    
    impl EFByteRepCompatible for EFI64 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("i64") },
                msg: String::from("Created byte rep from i64.")
            })
        }
        

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("i64") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"i64\")"), 
                    msg: String::from("Component is not set to i64.")
                });
            }

            let g_arr: [u8; 8] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 8-byte array for i64.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFI64(i64::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a i64.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFI128(pub i128);

    impl EFComponent for EFI128 {
        fn get_component_str(&self) -> String {
            String::from("i128")
        }
    }
    
    impl EFByteRepCompatible for EFI128 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("i128") },
                msg: String::from("Created byte rep from i128.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("i128") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"i128\")"), 
                    msg: String::from("Component is not set to i128.")
                });
            }

            let g_arr: [u8; 16] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 16-byte array for i128.")
                    });
                }
            };

            Ok(EFOk{
                value: EFI128(i128::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a i128.") 
            })
        }
    }
}

pub mod float {
    use super::{EFByteRep, EFComponent, EFByteRepCompatible};
    use super::{EFOk, EFError};

    // #[derive(Debug)]
    // pub struct EFF16(pub f16);

    #[derive(Debug)]
    pub struct EFF32(pub f32);

    impl EFComponent for EFF32 {
        fn get_component_str(&self) -> String {
            String::from("f32")
        }
    }
    
    impl EFByteRepCompatible for EFF32 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("f32") },
                msg: String::from("Created byte rep from f32.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("f32") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"f32\")"), 
                    msg: String::from("Component is not set to f32.")
                });
            }

            let g_arr: [u8; 4] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 4-byte array for f32.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFF32(f32::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a f32.") 
            })
        }
    }

    #[derive(Debug)]
    pub struct EFF64(pub f64);

    impl EFComponent for EFF64 {
        fn get_component_str(&self) -> String {
            String::from("f64")
        }
    }
    
    impl EFByteRepCompatible for EFF64 {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.0.to_be_bytes().to_vec(), component: String::from("f64") },
                msg: String::from("Created byte rep from f64.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("f64") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"f64\")"), 
                    msg: String::from("Component is not set to f64.")
                });
            }

            let g_arr: [u8; 8] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 8-byte array for f64.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFF64(f64::from_be_bytes(g_arr)),
                msg: String::from("Converted the byte rep into a f64.") 
            })
        }
    }

    // #[derive(Debug)]
    // pub struct EFF128(pub f128);

}

pub mod boolean {
    use super::{EFByteRep, EFComponent, EFByteRepCompatible};
    use super::{EFOk, EFError};

    #[derive(Debug)]
    pub struct EFBool(pub bool);

    impl EFComponent for EFBool {
        fn get_component_str(&self) -> String {
            String::from("bool")
        }
    }
    
    impl EFByteRepCompatible for EFBool {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            let value: EFByteRep = match self.0 {
                true => EFByteRep { bytes: vec![1u8], component: String::from("bool") },
                false => EFByteRep { bytes: vec![0u8], component: String::from("bool") }
            };

            Ok(EFOk{ value, msg: String::from("Created byte rep from bool.") })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("bool") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"bool\")"), 
                    msg: String::from("Component is not set to bool.")
                });
            }

            let g_arr: [u8; 1] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 1-byte array for bool.")
                    });
                }
            };

            Ok(EFOk{ 
                value: EFBool(match g_arr[0] {
                    1 => true,
                    0 => false,
                    _ => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("match g_arr[0]"), 
                            msg: String::from("Value inside byte rep was not a 1 or 0 for bool.")
                        });
                    }
                }),
                msg: String::from("Converted the byte rep into a u8.") 
            })
        }
    }
}

pub mod character {
    use super::{EFByteRep, EFComponent, EFByteRepCompatible};
    use super::{EFOk, EFError};
    use std::str::from_utf8;

    #[derive(Debug)]
    pub struct EFChar(pub char);

    impl EFComponent for EFChar {
        fn get_component_str(&self) -> String {
            String::from("char")
        }
    }
    
    impl EFByteRepCompatible for EFChar {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            let mut utf8_buffer: [u8; 4] = [0; 4];
            self.0.encode_utf8(&mut utf8_buffer);

            // Store the encoding and Unicode character length in the same vector
            let mut utf8_vec = utf8_buffer.to_vec();
            utf8_vec.push(self.0.len_utf8() as u8); // len_utf8() always returns a number from 1 to 4 inclusive

            Ok(EFOk{
                value: EFByteRep { bytes: utf8_vec, component: String::from("char") },
                msg: String::from("Created byte rep from char.")
            })
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("char") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"char\")"), 
                    msg: String::from("Component is not set to char.")
                });
            }

            let g_arr: [u8; 5] = match br.bytes.clone().try_into() {
                Ok(b) => b,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.clone().try_into()"), 
                        msg: String::from("Could not convert the byte rep into a 5-byte array for char.")
                    });
                }
            };

            // Get the length byte and create a string slice
            let utf8_len: usize = g_arr[4] as usize;
            let char_str: &str = match from_utf8(&g_arr[..utf8_len]) {
                Ok(s) => s,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("from_utf8(&g_arr[..utf8_len])"), 
                        msg: String::from("Could not create a UTF-8 string slice for char.")
                    });
                }
            };

            match char_str.chars().next() {
                Some(c) => Ok(EFOk{ 
                    value: EFChar(c), 
                    msg: String::from("Converted the byte rep into a char.")
                }),
                None => Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("char_str.chars().next()"), 
                    msg: String::from("Could not pull the next character for char.")
                })
            }
        }
    }
}
