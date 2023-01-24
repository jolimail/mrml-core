#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseAttributes))]
struct MjBreakpointAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "String::is_empty"))]
    width: String,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjBreakpoint {
    attributes: MjBreakpointAttributes,
}

impl MjBreakpoint {
    pub fn value(&self) -> &str {
        &self.attributes.width
    }
}
