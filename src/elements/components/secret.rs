use super::*;

#[derive(Debug)]
pub struct EFSecret {
    secret: EFString,
    salt: EFString,
    salt_generated_date: EFUTCTimestamp,
    version: EFVersion
}

impl EFComponent for EFSecret {
    type ComponentParams = (EFString, EFString, EFUTCTimestamp);

    fn new(params: Self::ComponentParams) -> Self {
        EFSecret {
            secret: params.0, salt: params.1, salt_generated_date: params.2, 
            version: EFSECRET_VERSION
        }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFSecret {
            secret: params.0, salt: params.1, salt_generated_date: params.2, 
            version
        }
    }

    fn get_component_str(&self) -> String {
        String::from(EFSECRET_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.version.clone()
    }

    fn clone_component(&self) -> Self {
        EFSecret {
            secret: self.secret.clone_component(),
            salt: self.salt.clone_component(),
            salt_generated_date: self.salt_generated_date.clone(),
            version: self.version.clone()
        }
    }

    fn handle_request(&self, request: &crate::elements::efid::EFQuery) -> crate::elements::efid::EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFSecret {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let secret_bytes: Vec<u8> = match self.secret.to_byte_rep() {
            Ok(nb) => nb.value.bytes,
            Err(e) => { return Err(e); }
        };
        let salt_bytes: Vec<u8> = match self.salt.to_byte_rep() {
            Ok(nb) => nb.value.bytes,
            Err(e) => { return Err(e); }
        };
        let salt_generated_date_byte: Vec<u8> = self.salt_generated_date.to_byte_vector();
        let byte_vectors: Vec<Vec<u8>> = vec![
            secret_bytes, salt_bytes, salt_generated_date_byte
        ];

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder{ 
            byte_vectors, version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFSECRET_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the attributes
        let secret: EFString = match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(index_object) => match EFString::from_byte_rep(&EFByteRep{ bytes: index_object.value }) {
                Ok(s) => s.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };
        let salt: EFString = match get_index_from_generic_vector(&byte_vectors, 1) {
            Ok(index_object) => match EFString::from_byte_rep(&EFByteRep{ bytes: index_object.value }) {
                Ok(s) => s.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };
        let salt_generated_date: EFUTCTimestamp = match get_index_from_generic_vector(&byte_vectors, 2) {
            Ok(index_object) => match EFUTCTimestamp::from_byte_vector(index_object.value) {
                Ok(ts) => ts.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFSecret { secret, salt, salt_generated_date, version },
            msg: String::from("Converted the byte rep into a secret.")
        })
    }
}
