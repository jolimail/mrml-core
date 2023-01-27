use super::{MjInclude, MjIncludeAttributes, MjIncludeChild};
use crate::prelude::parse::{Error, Parsable, Parser, ParserOptions};
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjIncludeChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            crate::mj_accordion::NAME => Ok(Self::MjAccordion(
                crate::mj_accordion::MjAccordion::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_button::NAME => Ok(Self::MjButton(crate::mj_button::MjButton::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_carousel::NAME => Ok(Self::MjCarousel(
                crate::mj_carousel::MjCarousel::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_column::NAME => Ok(Self::MjColumn(crate::mj_column::MjColumn::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_divider::NAME => Ok(Self::MjDivider(crate::mj_divider::MjDivider::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_group::NAME => Ok(Self::MjGroup(crate::mj_group::MjGroup::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_hero::NAME => Ok(Self::MjHero(crate::mj_hero::MjHero::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_image::NAME => Ok(Self::MjImage(crate::mj_image::MjImage::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_navbar::NAME => Ok(Self::MjNavbar(crate::mj_navbar::MjNavbar::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_raw::NAME => Ok(Self::MjRaw(crate::mj_raw::MjRaw::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_section::NAME => Ok(Self::MjSection(crate::mj_section::MjSection::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_social::NAME => Ok(Self::MjSocial(crate::mj_social::MjSocial::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_spacer::NAME => Ok(Self::MjSpacer(crate::mj_spacer::MjSpacer::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_table::NAME => Ok(Self::MjTable(crate::mj_table::MjTable::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_text::NAME => Ok(Self::MjText(crate::mj_text::MjText::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_wrapper::NAME => Ok(Self::MjWrapper(crate::mj_wrapper::MjWrapper::parse(
                tag, tokenizer, opts,
            )?)),
            _ => Ok(Self::Node(crate::node::Node::parse(tag, tokenizer, opts)?)),
        }
    }
}

#[derive(Debug)]
struct MjIncludeParser {
    opts: Rc<ParserOptions>,
    attributes: MjIncludeAttributes,
}

impl MjIncludeParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            attributes: MjIncludeAttributes::default(),
        }
    }
}

impl Parser for MjIncludeParser {
    type Output = MjInclude;

    fn build(self) -> Result<Self::Output, Error> {
        let child = self
            .opts
            .include_loader
            .load(&self.attributes.path, self.opts.clone())?;
        Ok(MjInclude {
            attributes: self.attributes,
            children: vec![child],
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "path" => {
                self.attributes.path = value.to_string();
            }
            _ => return Err(Error::UnexpectedAttribute(name.start())),
        }
        Ok(())
    }
}

impl Parsable for MjInclude {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjIncludeParser::new(opts).parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::prelude::parse::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parse::{Error, ParserOptions};

    #[test]
    fn basic_in_noop_resolver() {
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="basic.mjml" />
  </mj-body>
</mjml>
"#;
        let err = crate::mjml::Mjml::parse(json).unwrap_err();
        match err {
            Error::IncludeLoaderError(msg) => {
                assert_eq!(msg, "unable to resolve \"basic.mjml\"")
            }
            _ => panic!("expected a IncludeLoaderError"),
        }
    }

    #[test]
    fn basic_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
        let mut opts = ParserOptions::default();
        opts.include_loader = Box::new(resolver);
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="basic.mjml" />
  </mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse_with_options(json, Rc::new(opts)).unwrap();
        let body = root.children.body.unwrap();
        let include = body.children.first().unwrap().as_mj_include().unwrap();
        let _content = include.children.first().unwrap();
    }
}
