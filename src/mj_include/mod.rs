#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-include";

pub type Child = MjIncludeChild;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjIncludeChild {
    MjAccordion(crate::mj_accordion::MjAccordion),
    MjButton(crate::mj_button::MjButton),
    MjCarousel(crate::mj_carousel::MjCarousel),
    MjColumn(crate::mj_column::MjColumn),
    MjDivider(crate::mj_divider::MjDivider),
    MjGroup(crate::mj_group::MjGroup),
    MjHero(crate::mj_hero::MjHero),
    MjImage(crate::mj_image::MjImage),
    MjNavbar(crate::mj_navbar::MjNavbar),
    MjRaw(crate::mj_raw::MjRaw),
    MjSection(crate::mj_section::MjSection),
    MjSocial(crate::mj_social::MjSocial),
    MjSpacer(crate::mj_spacer::MjSpacer),
    MjTable(crate::mj_table::MjTable),
    MjText(crate::mj_text::MjText),
    MjWrapper(crate::mj_wrapper::MjWrapper),
    Node(crate::node::Node<crate::mj_body::MjBodyChild>),
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
pub struct MjIncludeAttributes {
    pub path: String,
}

impl MjIncludeAttributes {
    pub fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME", children = false))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjInclude {
    pub attributes: MjIncludeAttributes,
    pub children: Vec<Child>,
}
