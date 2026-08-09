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
        String::from(EFUSIZE_STR)
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

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFUSIZE_STR) {
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
pub struct EFU8(pub u8, pub EFVersion);

impl EFComponent for EFU8 {
    type ComponentParams = u8;

    fn new(params: Self::ComponentParams) -> Self {
        EFU8(params, EFU8_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFU8(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFU8_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFU8(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFU8 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let u8_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![u8_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFU8_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 1-byte array for u8.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFU8(u8::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a u8.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFU16(pub u16, pub EFVersion);

impl EFComponent for EFU16 {
    type ComponentParams = u16;

    fn new(params: Self::ComponentParams) -> Self {
        EFU16(params, EFU16_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFU16(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFU16_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFU16(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFU16 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let u16_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![u16_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFU16_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 2] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 2-byte array for u16.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFU16(u16::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a u16.")
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFU32(pub u32, pub EFVersion);

impl EFComponent for EFU32 {
    type ComponentParams = u32;

    fn new(params: Self::ComponentParams) -> Self {
        EFU32(params, EFU32_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFU32(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFU32_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFU32(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFU32 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let u32_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![u32_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFU32_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 4-byte array for u32.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFU32(u32::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a u32.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFU64(pub u64, pub EFVersion);

impl EFComponent for EFU64 {
    type ComponentParams = u64;

    fn new(params: Self::ComponentParams) -> Self {
        EFU64(params, EFU64_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFU64(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFU64_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFU64(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFU64 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let u64_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![u64_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFU64_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 8-byte array for u64.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFU64(u64::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a u64.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFU128(pub u128, pub EFVersion);

impl EFComponent for EFU128 {
    type ComponentParams = u128;

    fn new(params: Self::ComponentParams) -> Self {
        EFU128(params, EFU128_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFU128(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFU128_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFU128(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFU128 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let u128_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![u128_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFU128_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the value
        match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(b_vec) => {
                let b_arr: [u8; 16] = match b_vec.value.try_into() {
                    Ok(b) => b,
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("b_vec.value.try_into()"), 
                            msg: String::from("Could not convert the byte rep into a 16-byte array for u128.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFU128(u128::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a u128.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}
