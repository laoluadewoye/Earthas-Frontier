
use super::*;

#[derive(Debug)]
pub struct EFISize(pub isize, pub EFVersion);

impl EFComponent for EFISize {
    type ComponentParams = isize;

    fn new(params: Self::ComponentParams) -> Self {
        EFISize(params, EFISIZE_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFISize(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFISIZE_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFISize(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFISize {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let isize_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![isize_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFISIZE_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 8-byte array for isize.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFISize(isize::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a isize.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFI8(pub i8, pub EFVersion);

impl EFComponent for EFI8 {
    type ComponentParams = i8;

    fn new(params: Self::ComponentParams) -> Self {
        EFI8(params, EFI8_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFI8(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFI8_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFI8(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFI8 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let i8_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![i8_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFI8_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 1-byte array for i8.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFI8(i8::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a i8.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFI16(pub i16, pub EFVersion);

impl EFComponent for EFI16 {
    type ComponentParams = i16;

    fn new(params: Self::ComponentParams) -> Self {
        EFI16(params, EFI16_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFI16(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFI16_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFI16(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFI16 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let i16_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![i16_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFI16_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 2-byte array for i16.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFI16(i16::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a i16.")
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFI32(pub i32, pub EFVersion);

impl EFComponent for EFI32 {
    type ComponentParams = i32;

    fn new(params: Self::ComponentParams) -> Self {
        EFI32(params, EFI32_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFI32(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFI32_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFI32(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFI32 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let i32_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![i32_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFI32_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 4-byte array for i32.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFI32(i32::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a i32.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFI64(pub i64, pub EFVersion);

impl EFComponent for EFI64 {
    type ComponentParams = i64;

    fn new(params: Self::ComponentParams) -> Self {
        EFI64(params, EFI64_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFI64(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFI64_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFI64(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFI64 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let i64_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![i64_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFI64_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 8-byte array for i64.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFI64(i64::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a i64.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}

#[derive(Debug)]
pub struct EFI128(pub i128, pub EFVersion);

impl EFComponent for EFI128 {
    type ComponentParams = i128;

    fn new(params: Self::ComponentParams) -> Self {
        EFI128(params, EFI128_VERSION)
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFI128(params, version)
    }

    fn get_component_str(&self) -> String {
        String::from(EFI128_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.1.clone()
    }

    fn clone_component(&self) -> Self {
        EFI128(self.0, self.1.clone())
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFI128 {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.1.0, self.1.1, self.1.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let i128_bytes: Vec<u8> = self.0.to_be_bytes().to_vec();

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder { 
            byte_vectors: vec![i128_bytes], version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFI128_STR) {
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
                            msg: String::from("Could not convert the byte rep into a 16-byte array for i128.")
                        });
                    }
                };

                Ok(EFOk{ 
                    value: EFI128(i128::from_be_bytes(b_arr), version), 
                    msg: String::from("Converted the byte rep into a i128.") 
                })
            },
            Err(e) => { return Err(e); }
        }
    }
}
