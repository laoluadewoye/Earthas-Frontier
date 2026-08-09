use super::*;

#[derive(Debug)]
enum EFReferenceStatus {
    Permenent,
    Temporary(EFUTCTimestamp) // Future point in time
}

#[derive(Debug)]
pub struct EFReference {
    local_name: EFString,
    global_id: EFID,
    status: EFReferenceStatus
}
