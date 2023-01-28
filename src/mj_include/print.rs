use crate::prelude::hash::Map;

impl super::MjIncludeAttributes {
    pub fn as_map(&self) -> Map<String, String> {
        let mut res = Map::new();
        res.insert("path".to_string(), self.path.clone());
        if !self.kind.is_default() {
            res.insert("type".to_string(), self.kind.to_string());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::mj_include::{MjInclude, MjIncludeChild, MjIncludeKind};
    use crate::prelude::print::Print;

    #[test]
    fn simple() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let mut elt = MjInclude::default();
        elt.attributes.kind = MjIncludeKind::Html;
        elt.attributes.path = "memory:include.html".to_string();
        elt.children = vec![MjIncludeChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.dense_print(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
