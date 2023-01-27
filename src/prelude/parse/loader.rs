use super::ParserOptions;
use crate::prelude::parse::{is_element_start, next_token, Error, Parsable};
use std::rc::Rc;

pub enum IncludeLoaderError {
    Fetching(String),
    Parsing(String),
}

pub trait IncludeLoader: std::fmt::Debug {
    fn resolve(&self, path: &str) -> Result<String, String>;

    fn load(&self, path: &str, opts: Rc<ParserOptions>) -> Result<crate::mj_include::Child, Error> {
        let raw = self.resolve(path).map_err(Error::IncludeLoaderError)?;

        let mut tokenizer = xmlparser::Tokenizer::from(raw.as_ref());
        let token = next_token(&mut tokenizer)?;
        if let Some(tag) = is_element_start(&token) {
            crate::mj_include::Child::parse(*tag, &mut tokenizer, opts)
        } else {
            Err(Error::InvalidFormat)
        }
    }
}
