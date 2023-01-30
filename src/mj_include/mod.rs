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
    Comment(crate::comment::Comment),
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
    Text(crate::text::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "snake_case"))]
pub enum MjIncludeKind {
    Mjml,
    Html,
    Css { inline: bool },
}

impl ToString for MjIncludeKind {
    fn to_string(&self) -> String {
        match self {
            Self::Html => "html".to_string(),
            Self::Mjml => "mjml".to_string(),

            Self::Css { inline: _ } => "css".to_string(),
        }
    }
}

impl MjIncludeKind {
    fn is_default(&self) -> bool {
        matches!(self, Self::Mjml)
    }
}

impl Default for MjIncludeKind {
    fn default() -> Self {
        Self::Mjml
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
pub struct MjIncludeAttributes {
    pub path: String,
    #[cfg_attr(
        feature = "json",
        serde(
            default,
            rename = "type",
            skip_serializing_if = "MjIncludeKind::is_default"
        )
    )]
    pub kind: MjIncludeKind,
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
