use super::*;

#[derive(Debug, Clone)]
pub struct EFString(pub String, pub EFVersion);

impl EFComponent for EFString {
    type ComponentParams = String;

    fn create_new(params: Self::ComponentParams) -> Self {
        EFString(params, EFSTRING_VERSION)
    }
    
    fn create_from_compatible(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFString(params, version)
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
        EFSTRING_STR
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFString {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let type_vector: Vec<u8> = self.get_component_type().as_bytes().to_vec();

        // Create vectors from attributes
        let string_bytes: Vec<u8> = self.0.clone().into_bytes();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder {
            version_vector, type_vector, byte_vectors: vec![string_bytes]
        };
        builder.create_byte_rep()
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the version and vectors for attributes
        let (version, byte_vectors): (EFVersion, Vec<Vec<u8>>) = match 
        EFByteRepBuilder::validate_br_for_ver_and_attrs(byte_rep, EFSTRING_STR) {
            Ok(v_bvs) => (v_bvs.value.0, v_bvs.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the component
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => match get_string_from_byte_vector(&b_vec.value) {
                Ok(s) => Ok(EFOk{
                    value: EFString(s.value.to_string(), version), 
                    msg: String::from("Converted the byte rep into a string.")
                }),
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }
}
