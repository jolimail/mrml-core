#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::mj_include::{MjInclude, MjIncludeChild};
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
}
