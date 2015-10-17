use hyper;
use serde;
use std;

pub trait DocumentType {
    fn uri_path_component() -> Option<&'static str>;
}

pub struct NormalDocumentType;

impl DocumentType for NormalDocumentType {
    fn uri_path_component() -> Option<&'static str> { None }
}

pub struct DesignDocumentType;

impl DocumentType for DesignDocumentType {
    fn uri_path_component() -> Option<&'static str> { Some("_design") }
}

/// Document revision.
#[derive(Debug)]
pub struct Revision(String);

pub fn new_revision_from_string(rev: String) -> Revision {
    Revision(rev)
}

impl AsRef<str> for Revision {
    fn as_ref(&self) -> &str {
        let Revision(ref s) = *self;
        s
    }
}

impl Clone for Revision {
    fn clone(&self) -> Self {
        new_revision_from_string(self.to_string())
    }
}

impl std::fmt::Display for Revision {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl Ord for Revision {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl Eq for Revision {}

impl PartialEq for Revision {
    fn eq(&self, other: &Self) -> bool {
        let Revision(ref a) = *self;
        let Revision(ref b) = *other;
        a.eq(b)
    }
}

impl PartialOrd for Revision {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Revision(ref a) = *self;
        let Revision(ref b) = *other;
        a.partial_cmp(b)
    }
}

/// Document, including meta-information and content.
#[derive(Debug)]
pub struct Document<T: serde::Deserialize> {
    pub id: String,
    pub revision: Revision,
    pub content: T,
}

/// Construct a document URI.
pub fn new_uri<D>(
    base_uri: &hyper::Url,
    db_name: &str,
    doc_id: &str)
    -> hyper::Url
    where D: DocumentType
{
    let mut uri = base_uri.clone();
    {
        let p = uri.path_mut().unwrap();
        p.clear();
        p.push(db_name.to_string());
        for c in D::uri_path_component() {
            p.push(c.to_string());
        }
        p.push(doc_id.to_string());
    }
    uri
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_revision() {

        let r1 = super::new_revision_from_string("1-1234".to_string());

        let r2 = r1.clone();
        assert!(r1 == r2);
        assert!(!(r1 != r2));
        assert!(r1 <= r2);
        assert!(!(r1 < r2));
        assert!(r2 <= r1);
        assert!(!(r2 < r1));
        let r2 = super::new_revision_from_string("2-1234".to_string());
        assert!(!(r1 == r2));
        assert!(r1 != r2);
        assert!(r1 <= r2);
        assert!(r1 < r2);
        assert!(!(r2 <= r1));
        assert!(!(r2 < r1));
    }
}