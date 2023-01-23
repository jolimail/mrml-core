use super::MjAccordionChild;
use crate::mj_accordion_element::{MjAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjAccordionChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION_ELEMENT => Ok(MjAccordionElement::parse(tag, tokenizer)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}
