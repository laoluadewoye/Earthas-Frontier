use super::*;
use std::str::from_utf8 as str_slice_from_utf8;

#[derive(Debug)]
pub struct EFChar(pub char, pub EFVersion);

impl EFComponent for EFChar {
    type ComponentParams = char;

    fn new(params: Self::ComponentParams) -> Self {
        EFChar(params, EFCHAR_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFChar(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFCHAR_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFChar(self.0, self.1.clone())
    }

    fn upgrade_component(older_componet: EFComponentTuple) {

    }

    fn downgrade_component(&self) {
        
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFChar {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let mut char_buffer: [u8; 4] = [0; 4];
        self.0.encode_utf8(&mut char_buffer);
        let mut char_bytes: Vec<u8> = char_buffer.to_vec();
        char_bytes.push(self.0.len_utf8() as u8);

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![char_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFCHAR_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 5] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 5-byte array for char.")
                        });
                    }
                };

                let char_len: usize = b_arr[4] as usize;
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
