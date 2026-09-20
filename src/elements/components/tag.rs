use crate::elements::EFName;
use crate::elements::uri::EFURIString;

// Note: When creating a tagging system, some tags can imply other tags
#[derive(Debug)]
pub struct EFTag {
    name: EFName,
    tag_implies: Vec<EFURIString>
}
