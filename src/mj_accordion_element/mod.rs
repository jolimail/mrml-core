mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::hash::Map;

pub use children::MjAccordionElementChild;

pub const NAME: &str = "mj-accordion-element";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub struct MjAccordionElementChildren {
    pub title: Option<MjAccordionTitle>,
    pub text: Option<MjAccordionText>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordionElement {
    pub attributes: Map<String, String>,
    pub children: MjAccordionElementChildren,
}
