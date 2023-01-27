#[cfg(test)]
mod tests {
    use crate::{
        mj_button::MjButton,
        mj_include::{MjInclude, MjIncludeChild},
    };

    #[test]
    fn serialize() {
        let mut elt = MjInclude::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeChild::MjButton(MjButton::default())];
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-include","attributes":{"path":"memory:include.mjml"},"children":[{"type":"mj-button"}]}"#
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
