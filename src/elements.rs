pub mod core {
    use crate::utils::result::{EFOk, EFError};

    pub trait EFComponent {
        fn get_component_str(&self) -> String;
    }

    // Note: implementations should prefer big endian byte order
    pub struct EFByteRep {
        pub bytes: Vec<u8>,
        pub component: String
    }

    impl EFComponent for EFByteRep {
        fn get_component_str(&self) -> String {
            String::from("byte_rep")
        }
    }

    pub trait EFByteRepCompatible {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError>;
        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized;
    }

    impl EFByteRepCompatible for EFByteRep {
        // Essentially an byte rep cloner for symmetry with other EF components.
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            Ok(EFOk{
                value: EFByteRep { bytes: self.bytes.clone(), component: self.component.clone() },
                msg: String::from("Created byte rep from byte rep.")
            })
        }

        // Essentially an byte rep generator for symmetry with other EF components.
        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            Ok(EFOk{ 
                value: EFByteRep { 
                    bytes: br.bytes.clone(), 
                    component: br.component.clone() 
                },
                msg: String::from("Converted the byte rep into a byte rep.")
            })
        }
    }

    pub trait EFByteRepCompatibleEnum {
        fn get_byte_value(&self) -> u8;
        fn from_byte_value(v: u8) -> Result<EFOk<Self>, EFError> where Self: Sized;
    }
}

pub mod primitives {
    pub mod unsigned_int {
        use super::super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
        use crate::utils::result::{EFOk, EFError};

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
        use super::super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
        use crate::utils::result::{EFOk, EFError};

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
        use super::super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
        use crate::utils::result::{EFOk, EFError};

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
        use super::super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
        use crate::utils::result::{EFOk, EFError};

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
        use super::super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
        use crate::utils::result::{EFOk, EFError};
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
}

pub mod common {
    use super::core::{EFByteRep, EFComponent, EFByteRepCompatible};
    use crate::utils::result::{EFOk, EFError};

    impl EFComponent for String {
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
}

pub mod entity {
    use super::core::EFComponent;
    use crate::utils::std::get_hash;
    use chrono::Utc;
    use std::collections::HashMap;
    use crate::utils::result::{EFOk, EFError};

    #[derive(Debug)]
    pub struct EFEntity<T: EFComponent + ?Sized> {
        id: String,
        name: String,
        system: String,
        date_created: String,
        date_modified: String,
        component: Box<T>,
        component_type: String
    }

    impl <T: EFComponent + ?Sized> EFEntity<T> {
        pub fn new(name: String, system: String, component: Box<T>, salt: String, entity_hash: &String) -> EFEntity<T> {
            let timestamp: String = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let hash: String = match get_hash(vec![&system, &timestamp, &salt], entity_hash) {
                Ok(h) => h.value,
                Err(e) => { panic!("{}", &e.to_string().as_str()); }
            };
            let component_str: String = component.get_component_str();

            EFEntity { 
                id: hash,
                name,
                system, 
                date_created: timestamp.clone(),
                date_modified: timestamp.clone(),
                component: component,
                component_type: component_str
            }
        }

        pub fn get_id(&self) -> &String {
            &self.id
        }

        pub fn get_name(&self) -> &String {
            &self.name
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        pub fn get_system(&self) -> &String {
            &self.system
        }

        pub fn get_date_created(&self) -> &String {
            &self.date_created
        }

        pub fn get_date_modified(&self) -> &String {
            &self.date_modified
        }

        pub fn set_date_modified(&mut self, date_modified: String) {
            self.date_modified = date_modified;
        }

        pub fn get_component(&self) -> &Box<T> {
            &self.component
        }

        pub fn get_mutable_component(&mut self) -> &mut Box<T> {
            &mut self.component
        }

        pub fn get_component_type(&self) -> &String {
            &self.component_type
        }
    }

    pub trait EFEntityTracker {
        type EntityType;

        fn new() -> Self;
        fn add_entity(&mut self, entity: Self::EntityType) -> Option<Self::EntityType>;
        fn pop_entity(&mut self, entity_id: &str) -> Result<EFOk<Self::EntityType>, EFError>;
        fn get_entity(&self, entity_id: &str) -> Result<EFOk<&Self::EntityType>, EFError>;
        fn get_mut_entity(&mut self, entity_id: &str) -> Result<EFOk<&mut Self::EntityType>, EFError>;
    }

    pub struct EFStaticEntityTracker<T: EFComponent> {
        entities: HashMap<String, EFEntity<T>>
    }

    impl<T: EFComponent> EFEntityTracker for EFStaticEntityTracker<T> {
        type EntityType = EFEntity<T>;

        fn new() -> Self {
            EFStaticEntityTracker { entities: HashMap::new() }
        }

        fn add_entity(&mut self, entity: Self::EntityType) -> Option<Self::EntityType> {
            self.entities.insert(entity.get_id().clone(), entity)
        }

        fn pop_entity(&mut self, entity_id: &str) -> Result<EFOk<Self::EntityType>, EFError> {
            match self.entities.remove(entity_id) {
                Some(e) => Ok(EFOk { 
                    value: e, 
                    msg: format!("Popped entity {} from tracker.", entity_id)
                }),
                None => Err(EFError { 
                    function: String::from("pop_entity"), 
                    line: String::from("self.entities.remove(entity_id)"), 
                    msg: format!("Could not find entity {} in tracker to pop.", entity_id)
                })
            }
        }

        fn get_entity(&self, entity_id: &str) -> Result<EFOk<&Self::EntityType>, EFError> {
            match self.entities.get(entity_id) {
                Some(e) => Ok(EFOk { 
                    value: e, 
                    msg: format!("Got entity {} from tracker.", entity_id)
                }),
                None => Err(EFError { 
                    function: String::from("get_entity"),
                    line: String::from("self.entities.get(entity_id)"), 
                    msg: format!("Could not find entity {} in tracker to get.", entity_id)
                })
            }
        }

        fn get_mut_entity(&mut self, entity_id: &str) -> Result<EFOk<&mut Self::EntityType>, EFError> {
            match self.entities.get_mut(entity_id) {
                Some(e) => Ok(EFOk { 
                    value: e, 
                    msg: format!("Got mutable entity {} from tracker.", entity_id)
                }),
                None => Err(EFError { 
                    function: String::from("get_mut_entity"),
                    line: String::from("self.entities.get_mut(entity_id)"), 
                    msg: format!("Could not find entity {} in tracker to get as mutable.", entity_id)
                })
            }
        }
    }

    pub struct EFDynamicEntityTracker {
        entities: HashMap<String, EFEntity<dyn EFComponent>>
    }

    impl EFEntityTracker for EFDynamicEntityTracker {
        type EntityType = EFEntity<dyn EFComponent>;

        fn new() -> Self {
            EFDynamicEntityTracker { entities: HashMap::new() }
        }

        fn add_entity(&mut self, entity: Self::EntityType) -> Option<Self::EntityType> {
            self.entities.insert(entity.get_id().clone(), entity)
        }

        fn pop_entity(&mut self, entity_id: &str) -> Result<EFOk<Self::EntityType>, EFError> {
            match self.entities.remove(entity_id) {
                Some(e) => Ok(EFOk { 
                    value: e, 
                    msg: format!("Popped dynamic entity {} from tracker.", entity_id)
                }),
                None => Err(EFError { 
                    function: String::from("pop_entity"), 
                    line: String::from("self.entities.remove(entity_id)"), 
                    msg: format!("Could not find dynamic entity {} in tracker to pop.", entity_id)
                })
            }
        }

        fn get_entity(&self, entity_id: &str) -> Result<EFOk<&Self::EntityType>, EFError> {
            match self.entities.get(entity_id) {
                Some(e) => Ok(EFOk { 
                    value: e, 
                    msg: format!("Got dynamic entity {} from tracker.", entity_id)
                }),
                None => Err(EFError { 
                    function: String::from("get_entity"),
                    line: String::from("self.entities.get(entity_id)"), 
                    msg: format!("Could not find dynamic entity {} in tracker to get.", entity_id)
                })
            }
        }

        fn get_mut_entity(&mut self, entity_id: &str) -> Result<EFOk<&mut Self::EntityType>, EFError> {
            match self.entities.get_mut(entity_id) {
                Some(e) => Ok(EFOk {
                    value: e, 
                    msg: format!("Got mutable dynamic entity {} from tracker.", entity_id)
                }),
                None => Err(EFError {
                    function: String::from("get_mut_entity"),
                    line: String::from("self.entities.get_mut(entity_id)"), 
                    msg: format!("Could not find dynamic entity {} in tracker to get as mutable.", entity_id)
                })
            }
        }
    }
}

pub mod components {
    use super::core::{EFComponent, EFByteRep, EFByteRepCompatible, EFByteRepCompatibleEnum};
    use crate::utils::result::{EFOk, EFError};
    use crate::utils::std::{get_byte_rep_from_byte_vectors, get_byte_vectors_from_byte_rep};
    use super::primitives::unsigned_int::EFUSize;
    use super::entity::{EFStaticEntityTracker, EFDynamicEntityTracker};

    enum EFIdentityType {
        User,
        Role
    }

    impl EFByteRepCompatibleEnum for EFIdentityType {
        fn get_byte_value(&self) -> u8 {
            match self {
                EFIdentityType::User => 0u8,
                EFIdentityType::Role => 1u8
            }
        }

        fn from_byte_value(v: u8) -> Result<EFOk<Self>, EFError> {
            match v {
                0u8 => Ok(EFOk{ value: EFIdentityType::User, msg: String::from("Returned user.") }),
                1u8 => Ok(EFOk{ value: EFIdentityType::Role, msg: String::from("Returned role.") }),
                _ => Err(EFError { 
                    function: String::from("from_byte_value"), 
                    line: String::from("match v"), 
                    msg: String::from("Value must be within the range of 0 to 1 (inclusive) for identity type.")
                })
            }
        }
    }

    pub struct EFIdentity {
        name: String,
        identity_type: EFIdentityType,
        secret_entity: Option<String>
    }

    impl EFComponent for EFIdentity {
        fn get_component_str(&self) -> String {
            String::from("identity")
        }
    }

    impl EFByteRepCompatible for EFIdentity {
        fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
            // Create vectors from attributes
            let name_bytes: Vec<u8> = self.name.clone().into_bytes();
            let identity_type_bytes: Vec<u8> = vec![self.identity_type.get_byte_value()];
            let secret_entity_bytes: Vec<u8> = match &self.secret_entity {
                Some(se) => se.clone().into_bytes(),
                None => Vec::new()
            };

            // Return byte rep
            let mut byte_vector_set: Vec<Vec<u8>> = vec![
                name_bytes, identity_type_bytes, secret_entity_bytes
            ];
            get_byte_rep_from_byte_vectors(&mut byte_vector_set, self.get_component_str())
        }

        fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
            if !br.component.eq("identity") {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("!br.component.eq(\"identity\")"), 
                    msg: String::from("Component is not set to identity.")
                });
            }

            // Get the vectors for each attribute
            let identity_bytes: Vec<Vec<u8>> = match get_byte_vectors_from_byte_rep(br, 3) {
                Ok(bv) => bv.value,
                Err(e) => { return Err(e); }
            };

            // Create the attributes
            let name: String = match String::from_utf8(identity_bytes[0].clone()) {
                Ok(s) => s,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("String::from_utf8(name_bytes)"), 
                        msg: String::from("Passed in byte rep is not compatible with UTF-8 for identity's name.")
                    });
                }
            };
            let identity_type: EFIdentityType = match EFIdentityType::from_byte_value(identity_bytes[1][0]) {
                    Ok(i) => i.value,
                    Err(e) => { return Err(e); }
            };
            let secret_entity: Option<String> = match identity_bytes[2].is_empty() {
                true => None,
                false => match String::from_utf8(identity_bytes[2].clone()) {
                    Ok(s) => Some(s),
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("String::from_utf8(secret_entity_bytes)"), 
                            msg: String::from("Passed in byte rep is not compatible with UTF-8 for identity's secret entity.")
                        });
                    }
                }
            };

            // Create the final product if everything passes
            Ok(EFOk{
                value: EFIdentity { name, identity_type, secret_entity },
                msg: String::from("Converted the byte rep into an identity.")
            })
        }
    }

    pub struct EFSystem {
        salt: EFUSize,
        absolute_path: String,
        identities: EFStaticEntityTracker<EFIdentity>,
        secrets: EFStaticEntityTracker<EFSecret>,
        hooks: EFStaticEntityTracker<EFHook>,
        references: EFStaticEntityTracker<EFReference>,
        connections: EFStaticEntityTracker<EFConnection>,
        dataflows: EFStaticEntityTracker<EFDataFlow>,
        cache: EFCache,
        entities: EFDynamicEntityTracker,
        rules: EFStaticEntityTracker<EFRule>,
    }
}

pub struct GlobalState;
