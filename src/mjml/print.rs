use super::MJML;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for MJML {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        let mut res = print::open(
            super::NAME,
            Some(&self.attributes),
            false,
            pretty,
            level,
            indent_size,
        );
        res.push_str(
            &self
                .head()
                .as_ref()
                .map(|h| h.print(pretty, level + 1, indent_size))
                .unwrap_or_default(),
        );
        res.push_str(
            &self
                .body()
                .as_ref()
                .map(|b| b.print(pretty, level + 1, indent_size))
                .unwrap_or_default(),
        );
        res.push_str(&print::close(super::NAME, pretty, level, indent_size));
        res
    }
}

print_display!(MJML);

#[cfg(test)]
mod tests {
    use crate::mjml::{MJMLChildren, MJML};
    use crate::prelude::hash::Map;
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = MJML::default();
        assert_eq!("<mjml></mjml>", format!("{}", item));
    }

    #[test]
    fn with_lang() {
        let mut item = MJML::default();
        item.attributes.insert("lang".to_string(), "fr".to_string());
        assert_eq!("<mjml lang=\"fr\"></mjml>", format!("{}", item));
    }

    #[test]
    fn with_body() {
        let item = MJML {
            attributes: Map::default(),
            children: MJMLChildren {
                head: None,
                body: Some(crate::mj_body::MJBody::default()),
            },
        };
        assert_eq!("<mjml><mj-body></mj-body></mjml>", item.dense_print());
        assert_eq!(
            "<mjml>\n  <mj-body>\n  </mj-body>\n</mjml>\n",
            item.pretty_print()
        );
    }
}
