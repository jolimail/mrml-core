use std::rc::Rc;

use super::MjNavbarChild;
use crate::mj_navbar_link::{MjNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::prelude::parse::{Error, Parsable, ParserOptions};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjNavbarChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_NAVBAR_LINK => Ok(MjNavbarLink::parse(tag, tokenizer, opts)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
