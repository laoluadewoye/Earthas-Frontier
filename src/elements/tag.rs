use crate::elements::EFName;
use crate::elements::uri::EFURIString;

// Note: When creating a tagging system, some tags can imply other tags
// Note: Tags should live as an entity and as a reference inside an entity
#[derive(Debug)]
pub struct EFTag {
    name: EFName,
    tag_implies: Vec<EFURIString>,
    entities_tagged: Vec<EFURIString>
}

#[derive(Debug)]
pub struct EFTagTrackerEntry {
    name: EFName,
    uri: EFURIString
}

// Note: Tag tracker should be a try tree
pub trait EFTagTracker {
    fn new() -> Self;

    // Tag tracker metadata
    
}
