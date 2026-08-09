use super::*;

#[derive(Debug)]
pub struct EFSecret {
    secret: String,
    salt: String,
    salt_generated_date: String
}
