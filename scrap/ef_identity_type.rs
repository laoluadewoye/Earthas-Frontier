#[derive(Debug, Clone)]
enum EFIdentityType {
    User,
    Role
}

impl EFByteRepCompatibleEnum for EFIdentityType where Self: Sized {
    fn get_byte_vec(&self) -> Vec<u8> {
        match self {
            EFIdentityType::User => vec![0u8],
            EFIdentityType::Role => vec![1u8]
        }
    }

    fn from_byte_vec(byte_vec: &Vec<u8>) -> Result<EFOk<Self>, EFError> where Self: Sized {
        let type_byte: u8 = match get_index_from_generic_vector(byte_vec, 0) {
            Ok(index_object) => index_object.value,
            Err(e) => { return Err(e); }
        };

        match type_byte {
            0u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFIdentityType::User, msg: String::from("Returned user.")
            }),
            1u8 if byte_vec.len() == 1 => Ok(EFOk{
                value: EFIdentityType::Role, msg: String::from("Returned role.")
            }),
            _ => Err(EFError { 
                function: String::from("from_byte_vec"), 
                line: String::from("match type_byte"), 
                msg: String::from("Value must be within the range of 0 to 1 (inclusive)
                    and without any extra data for EFIdentityType.")
            })
        }
    }
}
