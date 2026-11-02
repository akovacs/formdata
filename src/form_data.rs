// Copyright © 2015 by Michael Dilger (of New Zealand)
// This code is licensed under the MIT license (see LICENSE-MIT for details)

use mime_multipart::{Node, Part, FilePart};
use hyper::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE};
use headers::{ContentDisposition, ContentType, HeaderMapExt};
use mime::{Mime, TopLevel, SubLevel};
use crate::error::Error;

/// The extracted text fields and uploaded files from a `multipart/form-data` request.
///
/// Use `parse_multipart` to devise this object from a request.
#[derive(Clone, Debug, PartialEq)]
pub struct FormData {
    /// Name-value pairs for plain text fields. Technically, these are form data parts with no
    /// filename specified in the part's `Content-Disposition`.
    pub fields: Vec<(String, String)>,
    /// Name-value pairs for temporary files. Technically, these are form data parts with a filename
    /// specified in the part's `Content-Disposition`.
    pub files: Vec<(String, FilePart)>,
}

impl FormData {
    pub fn new() -> FormData {
        FormData { fields: vec![], files: vec![] }
    }

    /// Create a mime-multipart Vec<Node> from this FormData
    pub fn to_multipart(&self) -> Result<Vec<Node>, Error> {
        // Translate to Nodes
        let mut nodes: Vec<Node> = Vec::with_capacity(self.fields.len() + self.files.len());

        for &(ref name, ref value) in &self.fields {
            let mut headersmap = HeaderMap::new();
            //headersmap.typed_insert(ContentType::text_utf8());
            headersmap.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            let val_str = format!(r#"form-data; name="name"; filename="{}""#, name);
            if let Ok(hv) = HeaderValue::from_str(&val_str) {
                headersmap.insert(CONTENT_DISPOSITION, hv);
            }
            nodes.push( Node::Part( Part {
                headers: headersmap,
                body: value.as_bytes().to_owned(),
            }));
        }

        for &(ref name, ref filepart) in &self.files {
            let mut filepart = filepart.clone();
            // We leave all headers that the caller specified, except that we rewrite
            // Content-Disposition.
            // Until hyper v0.11.0, headers.remove used to return a bool, now it
            // returns Option<H>. Repeatedly remove until None is returned.
            // Note that Headers will be replaced by HeaderMap in hyper 0.12,
            // fix this code then.
            while filepart.headers.remove(CONTENT_DISPOSITION).is_some() { };
            let filename = match filepart.path.file_name() {
                Some(fname) => fname.to_string_lossy().into_owned(),
                None => return Err(Error::NotAFile),
            };
            // let content_disposition = ContentDisposition::form_data()
            //    .with_name("name")
            //    .with_filename(name.clone());
            // filepart.headers.insert(CONTENT_DISPOSITION, ContentDisposition);
            //
            let val_str = format!(r#"form-data; name="{}"; filename="{}""#, name.clone(), filename);
            if let Ok(hv) = HeaderValue::from_str(&val_str) {
                filepart.headers.insert(CONTENT_DISPOSITION, hv);
            }
            nodes.push( Node::File( filepart ) );
        }

        Ok(nodes)
    }
}
