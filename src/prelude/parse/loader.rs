use super::ParserOptions;
use crate::{
    comment::Comment,
    prelude::parse::{next_token, Error, Parsable},
    text::Text,
};
use std::rc::Rc;
use xmlparser::Token;

pub enum IncludeLoaderError {
    Fetching(String),
    Parsing(String),
}

pub trait IncludeLoader: std::fmt::Debug {
    fn resolve(&self, path: &str) -> Result<String, String>;
}

pub fn parse<T: Parsable + From<Comment> + From<Text>>(
    include: &str,
    opts: Rc<ParserOptions>,
) -> Result<T, Error> {
    let mut tokenizer = xmlparser::Tokenizer::from(include);
    let token = next_token(&mut tokenizer)?;
    match token {
        Token::Comment { text, span: _ } => Ok(Comment::from(text.to_string()).into()),
        Token::Text { text } => Ok(Text::from(text.to_string()).into()),
        Token::ElementStart { local, .. } => T::parse(local, &mut tokenizer, opts),
        _ => Err(Error::InvalidFormat),
    }
}
