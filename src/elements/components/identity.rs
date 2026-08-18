use super::*;

#[derive(Debug)]
pub struct EFIdentity {
    name: EFString,
    roles: Vec<EFURIString>,
    secret: Option<EFURIString>,
    version: EFVersion
}

impl EFComponent for EFIdentity {
    type ComponentParams = (EFString, Vec<EFURIString>, Option<EFURIString>);

    fn new(params: Self::ComponentParams) -> Self where Self: Sized {
        EFIdentity { 
            name: params.0, roles: params.1, secret: params.2, 
            version: EFIDENTITY_VERSION
        }
    }

    fn build(params: Self::ComponentParams, version: EFVersion) -> Self where Self: Sized {
        EFIdentity { 
            name: params.0, roles: params.1, secret: params.2, 
            version
        }
    }

    fn get_component_str(&self) -> String {
        String::from(EFIDENTITY_STR)
    }

    fn get_component_version(&self) -> EFVersion {
        self.version.clone()
    }

    fn clone_component(&self) -> Self where Self: Sized {
        EFIdentity {
            name: self.name.clone_component(), 
            roles: self.roles.clone(), 
            secret: self.secret.clone(), 
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

impl EFByteRepCompatible for EFIdentity {
    fn to_byte_rep(&self) -> Result<EFOk<EFByteRep>, EFError> {
        // Create vectors from metadata
        let version_vector: Vec<u8> = vec![self.version.0, self.version.1, self.version.2];
        let component_vector: Vec<u8> = self.get_component_str().into_bytes();

        // Create vectors from attributes
        let name_bytes: Vec<u8> = match self.name.to_byte_rep() {
            Ok(nb) => nb.value.bytes,
            Err(e) => { return Err(e); }
        };
        let secret_bytes: Vec<u8> = match &self.secret {
            Some(se) => se.clone().0.into_bytes(),
            None => Vec::new()
        };
        let mut byte_vectors: Vec<Vec<u8>> = vec![
            name_bytes, secret_bytes
        ];
        for role in &self.roles {
            byte_vectors.push(role.clone().0.into_bytes())
        }

        // Return byte rep
        let mut builder: EFByteRepBuilder = EFByteRepBuilder{ 
            byte_vectors, version_vector, component_vector
        };
        get_byte_rep_from_builder(&mut builder)
    }

    fn from_byte_rep(byte_rep: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        // Get the byte vectors and version
        let (byte_vectors, version) = match get_byte_vectors_and_version_from_byte_rep(byte_rep, EFIDENTITY_STR) {
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
        let secret: Option<EFURIString> = match get_index_from_generic_vector(&byte_vectors, 1) {
            Ok(index_object) => match index_object.value.is_empty() {
                true => None,
                false => match get_string_from_byte_vector(index_object.value) {
                    Ok(s) => Some(EFURIString(s.value)),
                    Err(e) => { return Err(e); }
                }
            },
            Err(e) => { return Err(e); }
        };

        let mut roles: Vec<EFURIString> = Vec::new();
        for i in 2..byte_vectors.len() {
            match get_string_from_byte_vector(byte_vectors[i].clone()) {
                Ok(s) => roles.push(EFURIString(s.value)),
                Err(e) => { return Err(e); }
            }
        }
        
        // Create the final product if everything passes
        Ok(EFOk{
            value: EFIdentity::build((name, roles, secret), version),
            msg: String::from("Converted the byte rep into an identity.")
        })
    }
}
