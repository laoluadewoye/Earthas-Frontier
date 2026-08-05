impl EFByteRepCompatible for EFIdentity {
    fn to_byte_rep(&self) -> EFByteRep {
        // Create vectors from attributes
        let mut name_bytes: Vec<u8> = self.name.clone().into_bytes();
        let mut identity_type_bytes: Vec<u8> = vec![self.identity_type.get_byte_value()];
        let mut secret_entity_bytes: Vec<u8> = match &self.secret_entity {
            Some(se) => se.clone().into_bytes(),
            None => Vec::new()
        };

        // Return byte rep
        get_byte_rep_from_byte_vectors(vec![name_bytes, identity_type_bytes, secret_entity_bytes], self.get_component_str())

        // Calculate offsets
        let name_bytes_start: u8 = 3u8;
        let identity_type_bytes_start: u8 = name_bytes_start + name_bytes.len() as u8;
        let secret_entity_bytes_start: u8 = match self.secret_entity {
            Some(_) => identity_type_bytes_start + 1u8,
            None => 0u8 // Default value of 0 in starters
        };

        // Create final vector
        let mut byte_rep: Vec<u8> = vec![
            name_bytes_start,
            identity_type_bytes_start,
            secret_entity_bytes_start
        ];
        byte_rep.append(&mut name_bytes);
        byte_rep.append(&mut identity_type_bytes);
        byte_rep.append(&mut secret_entity_bytes);

        // Return byte rep
        EFByteRep { bytes: byte_rep, component: self.get_component_str() }
    }

    fn from_byte_rep(br: &EFByteRep) -> Result<EFOk<Self>, EFError> {
        if !br.component.eq("identity") {
            return Err(EFError{
                function: String::from("from_byte_rep"), 
                line: String::from("!br.component.eq(\"identity\")"), 
                msg: String::from("Component is not set to identity.")
            });
        }

        // Get the start points v2
        let (name_bytes_start, 
            identity_type_bytes_start, 
            secret_entity_bytes_start
        ) = match br.bytes.get(0..3) {
            Some(&[n, i, s]) => (n as usize, i as usize, s as usize),
            _ => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("br.bytes.get(0..3)"), 
                    msg: String::from("One or more values missing at first 3 indexes for identity.")
                });
            }
        };

        // Create the attributes
        let name: String = match br.bytes.get(name_bytes_start..identity_type_bytes_start) {
            Some(n) => match String::from_utf8(n.to_vec()) {
                Ok(s) => s,
                Err(_) => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.get(name_bytes_start..identity_type_bytes_start)"), 
                        msg: String::from("Passed in byte rep is not compatible with UTF-8 for identity.")
                    });
                }
            },
            None => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("br.bytes.get(name_bytes_start..identity_type_bytes_start)"), 
                    msg: String::from("Obtaining byte slice for name failed.")
                });
            }
        };
        let identity_type: EFIdentityType = match br.bytes.get(identity_type_bytes_start) {
            Some(i) => match EFIdentityType::from_byte_value(i.to_owned()) {
                Ok(i) => i.value,
                Err(e) => { return Err(e); }
            },
            None => {
                return Err(EFError{
                    function: String::from("from_byte_rep"), 
                    line: String::from("br.bytes.get(identity_type_bytes_start)"), 
                    msg: String::from("Obtaining byte for identity failed.")
                });
            }
        };
        let secret_entity: Option<String> = match secret_entity_bytes_start {
            0 => None,
            _ => match br.bytes.get(secret_entity_bytes_start..) {
                Some(s) => match String::from_utf8(s.to_vec()) {
                    Ok(s) => Some(s),
                    Err(_) => {
                        return Err(EFError{
                            function: String::from("from_byte_rep"), 
                            line: String::from("String::from_utf8(s.to_vec())"), 
                            msg: String::from("Passed in byte rep is not compatible with UTF-8 for secret entity.")
                        });
                    }
                },
                None => {
                    return Err(EFError{
                        function: String::from("from_byte_rep"), 
                        line: String::from("br.bytes.get(secret_entity_bytes_start..)"), 
                        msg: String::from("Obtaining byte slice for secret entity failed.")
                    });
                }
            }
        };

        // Create the final product if everything passes
        Ok(EFOk{
            value: EFIdentity { name, identity_type, secret_entity },
            msg: String::from("Converted the byte rep into an identity.")
        })
    }
}
