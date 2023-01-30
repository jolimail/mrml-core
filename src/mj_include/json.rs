#[cfg(test)]
mod tests {
    use crate::{
        mj_button::MjButton,
        mj_include::{MjInclude, MjIncludeChild, MjIncludeKind},
        text::Text,
    };

    #[test]
    fn serialize_mjml() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeChild::MjButton(MjButton::default())];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn serialize_html() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.html".to_string();
        elt.attributes.kind = MjIncludeKind::Html;
        elt.children = vec![MjIncludeChild::MjButton(MjButton::default())];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.html","type":"html"},"children":[{"type":"mj-button"}]}"#
        );
    }

    #[test]
    fn serialize_css() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.css".to_string();
        elt.attributes.kind = MjIncludeKind::Css { inline: false };
        elt.children = vec![MjIncludeChild::Text(Text::from("Hello World!"))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":false}}},"children":["Hello World!"]}"#
        );
    }

    #[test]
    fn serialize_css_inline() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.css".to_string();
        elt.attributes.kind = MjIncludeKind::Css { inline: true };
        elt.children = vec![MjIncludeChild::Text(Text::from("Hello World!"))];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.css","type":{"css":{"inline":true}}},"children":["Hello World!"]}"#
        );
    }

    #[test]
    fn deserialize() {
        let json = r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#;
        let res: MjInclude = serde_json::from_str(json).unwrap();
        assert_eq!(res.attributes.path, "memory:include.mjml");
        assert!(!res.children.is_empty());
    }
}
