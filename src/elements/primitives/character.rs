use super::*;
use std::str::from_utf8 as str_slice_from_utf8;

const EFCHAR_BUFFER_LEN: usize = 5;

#[derive(Debug, Clone)]
pub struct EFChar(pub char, pub EFVersion);

impl EFComponent for EFChar {
    type ComponentParams = char;

    fn create_new(params: Self::ComponentParams) -> Self {
        EFChar(params, EFCHAR_VERSION)
    }

    fn create_from_compatible(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFChar(params, version)
    }

    fn create_from_older(older_component: &EFComponentTuple) -> Option<Self> where Self: Sized {
        None
    }

    fn get_component_as_older(&self, old_version: &EFVersion) -> EFComponentTuple {
        EFComponentTuple
    }

    fn get_component_version(&self) -> &EFVersion {
        &self.1
    }

    fn get_component_type(&self) -> &str {
        EFCHAR_STR
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}


impl EFByteRepCompatible for EFChar {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let type_vector: Vec<u8> = self.get_component_type().as_bytes().to_vec();

        // Create vectors from attributes
        let mut char_buffer: [u8; EFCHAR_BUFFER_LEN] = [0; EFCHAR_BUFFER_LEN];
        let char_buffer_subset = self.0.encode_utf8(&mut char_buffer);
        char_buffer[EFCHAR_BUFFER_LEN-1] = char_buffer_subset.len() as u8;
        let char_bytes: Vec<u8> = char_buffer.to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder {
            version_vector, type_vector, byte_vectors: vec![char_bytes]
        };
        builder.create_byte_rep()
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the version and vectors for attributes
        let (version, byte_vectors): (EFVersion, Vec<Vec<u8>>) = match 
        EFByteRepBuilder::validate_br_for_ver_and_attrs(byte_rep, EFCHAR_STR) {
            Ok(v_bvs) => (v_bvs.value.0, v_bvs.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the component
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; EFCHAR_BUFFER_LEN] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 5-byte array for char.")
                        });
                    }
                };

                let char_len: usize = b_arr[EFCHAR_BUFFER_LEN-1] as usize;
                let char_str: &str = match str_slice_from_utf8(&b_arr[..char_len]) {
                    Ok(s_slice) => s_slice,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("str_slice_from_utf8(&b_arr[..char_len])"), 
                            msg: String::from("Could not create a UTF-8 string slice for char.")
                        });
                    }
                };

                match char_str.chars().next() {
                    Some(c) => Ok(EFOk{ 
                        value: EFChar(c, version), 
                        msg: String::from("Converted the byte rep into a char.")
                    }),
                    None => Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("char_str.chars().next()"), 
                        msg: String::from("Could not pull the next character for char.")
                    })
                }
            },
            Err(e) => { return Err(e); }
        }
    }
}
