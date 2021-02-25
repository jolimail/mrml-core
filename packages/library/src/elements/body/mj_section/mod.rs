mod parser;
mod renderer;

use crate::elements::body::mj_body::children::MJBodyChild;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-section";

const DEFAULT_BACKGROUND_POSITION: &str = "top center";

#[derive(Clone, Debug)]
pub struct MJSection {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJBodyChild>,
}