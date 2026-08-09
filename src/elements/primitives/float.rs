
use super::*;

// #[derive(Debug)]
// pub struct EFF16(pub f16);

#[derive(Debug)]
pub struct EFF32(pub f32, pub EFVersion);

impl EFComponent for EFF32 {
    type ComponentParams = f32;

    fn new(params: Self::ComponentParams) -> Self {
        EFF32(params, EFF32_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFF32(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFF32_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFF32(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFF32 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let f32_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![f32_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFF32_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 4] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 4-byte array for f32.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFF32(f32::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a f32.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFF64(pub f64, pub EFVersion);

impl EFComponent for EFF64 {
    type ComponentParams = f64;

    fn new(params: Self::ComponentParams) -> Self {
        EFF64(params, EFF64_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFF64(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFF64_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFF64(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFF64 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let f64_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![f64_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFF64_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 8] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 8-byte array for f64.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFF64(f64::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a f64.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

// #[derive(Debug)]
// pub struct EFF128(pub f128);
