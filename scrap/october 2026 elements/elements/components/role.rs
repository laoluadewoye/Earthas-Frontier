use super::*;

#[derive(Debug)]
pub struct EFRole {
    name: EFString,
    expiration_time: Option<EFUTCTimestamp>,
    version: EFVersion
}

impl EFComponent for EFRole {
    type ComponentParams = (EFString, Option<EFUTCTimestamp>);

    fn new(params: Self::ComponentParams) -> Self {
        EFRole { name: params.0, expiration_time: params.1, version: EFROLE_VERSION }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFRole { name: params.0, expiration_time: params.1, version }
    }

    fn get_component_str(&self) -> String {
        String::from(EFROLE_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.version.clone()
    }

    fn clone_component(&self) -> Self {
        EFRole {
            name: self.name.clone_component(),
            expiration_time: self.expiration_time.clone(), 
            version: self.version.clone()
        }
    }

    fn upgrade_component(older_componet: EFComponentTuple) {

    }

    fn downgrade_component(&self) {
        
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFRole {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let name_bytes: Vec<u8> = match self.name.to_byte_rep() {
            Ok(nb) => nb.value.bytes,
            Err(e) => { return Err(e); }
        };
        let expiration_time_bytes: Vec<u8> = match &self.expiration_time {
            Some(et) => et.to_byte_vector(),
            None =>  Vec::new()
        };
        let byte_vectors: Vec<Vec<u8>> = vec![
            name_bytes, expiration_time_bytes
        ];

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder{ 
            byte_vectors, version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFROLE_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the attributes
        let name: EFString = match get_index_from_generic_vector(&byte_vectors, 0) {
            Ok(index_object) => match EFString::from_byte_rep(&EFByteRep{ bytes: index_object.value }) {
                Ok(s) => s.value,
                Err(e) => { return Err(e); }
            },
            Err(e) => { return Err(e); }
        };
        let expiration_time: Option<EFUTCTimestamp> = match get_index_from_generic_vector(&byte_vectors, 1) {
            Ok(index_object) => match index_object.value.is_empty() {
                true => None,
                false => match EFUTCTimestamp::from_byte_vector(index_object.value) {
                    Ok(ts) => Some(ts.value),
                    Err(e) => { return Err(e); }
                }
            },
            Err(e) => { return Err(e); }
        };

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFRole::build((name, expiration_time), version),
            msg: String::from("Converted the byte rep into a role.")
        })
    }
}

pub struct EFRoleVector {
    roles: Vec<EFRole>,
    version: EFVersion
}

impl EFComponent for EFRoleVector {
    type ComponentParams = Vec<EFRole>;

    fn new(params: Self::ComponentParams) -> Self {
        EFRoleVector { roles: params, version: EFROLEVECTOR_VERSION }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self {
        EFRoleVector { roles: params, version }
    }

    fn get_component_str(&self) -> String {
        String::from(EFROLEVECTOR_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.version.clone()
    }

    fn clone_component(&self) -> Self {
        EFRoleVector {
            roles: self.roles.iter().map(|role| role.clone_component()).collect(), 
            version: self.version.clone()
        }
    }

    fn upgrade_component(older_componet: EFComponentTuple) {

    }

    fn downgrade_component(&self) {
        
    }

    fn handle_request(&self, request: &EFQuery) -> EFResponse {
        EFResponse
    }
}

impl EFByteRepCompatible for EFRoleVector {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attribute
        let mut byte_vectors: Vec<Vec<u8>> = Vec::new();
        for role in &self.roles {
            match role.clone_component().to_byte_rep() {
                Ok(r) => byte_vectors.push(r.value.bytes),
                Err(e) => { return Err(e); }
            }
        }

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder{ 
            byte_vectors, version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> where Self: Sized {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFROLEVECTOR_STR) {
            Ok(bv_v) => (bv_v.value.0, bv_v.value.1),
            Err(e) => { return Err(e); }
        };

        // Create the attributes
        let mut roles: Vec<EFRole> = Vec::new();
        for byte_vector in byte_vectors {
            match EFRole::from_byte_rep(&EFByteRep { bytes: byte_vector }) {
                Ok(r) => roles.push(r.value),
                Err(e) => { return Err(e); }
            }
        }

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFRoleVector { roles, version }, 
            msg: String::from("Converted the byte rep into a role vector.")
        })
    }
}
