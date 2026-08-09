use super::*;

pub struct EFString(pub String, pub EFVersion);

impl EFComponent for EFString {
    type ComponentParams = String;

    fn new(params: Self::ComponentParams) -> Self {
        EFString(params, EFSTRING_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFString(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFSTRING_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFString(self.0.clone(), self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFString {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let string_bytes: Vec<u8> = self.0.clone().into_bytes();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![string_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFSTRING_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => match get_string_from_byte_vector(b_vec.value) {
                Ok(s) => Ok(EFOk{
                    value: EFString(s.value, version), 
                    msg: String::from("Converted the byte rep into a string.")
                }),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}
