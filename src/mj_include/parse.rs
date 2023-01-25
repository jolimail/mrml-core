#[cfg(test)]
mod tests {
    #[test]
    fn basic_in_memory_resolver() {
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="memory:basic.mjml" />
  </mj-body>
</mjml>
"#;
        assert!(crate::mjml::Mjml::parse(json).is_ok());
    }
}
