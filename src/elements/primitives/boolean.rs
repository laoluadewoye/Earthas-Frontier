use super::*;

#[derive(Debug)]
pub struct EFBool(pub bool, pub EFVersion);

impl EFComponent for EFBool {
    type ComponentParams = bool;

    fn new(params: Self::ComponentParams) -> Self {
        EFBool(params, EFBOOL_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFBool(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFBOOL_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFBool(self.0, self.1.clone())
    }

    fn upgrade_component(older_componet: EFComponentTuple) {

    }

    fn downgrade_component(&self) {
        
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFBool {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let bool_bytes: Vec<u8> = match self.0 {
            true => vec![1u8],
            false => vec![0u8]
        };

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![bool_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFBOOL_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 1] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 1-byte array for bool.")
                        });
                    }
                };

                match b_arr[0] {
                    1 => Ok(EFOk{
                        value: EFBool(true, version), 
                        msg: String::from("Converted the byte rep into a boolean true.")
                    }),
                    0 => Ok(EFOk{
                        value: EFBool(false, version), 
                        msg: String::from("Converted the byte rep into a boolean false.")
                    }),
                    _ => Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("b_arr[0]"), 
                        msg: String::from("Value inside byte rep was not a 1 or 0 for bool.")
                    })
                }
            },
            Err(e) => { return Err(e); }
        }
    }
}
