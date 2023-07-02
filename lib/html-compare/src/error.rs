use crate::token::{Attribute, ElementEnd, ElementStart};
use colored::Colorize;
use htmlparser::{StrSpan, Token};
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum ErrorKind<'a> {
    ExpectedElementNotFound {
        expected_parent: StrSpan<'a>,
        expected_element: Token<'a>,
        generated_parent: StrSpan<'a>,
    },
    UnexpectedElementFound {
        generated: Token<'a>,
    },
    ElementMismatch {
        expected: Token<'a>,
        generated: Token<'a>,
    },
    EndOfElementMismatch {
        expected: ElementEnd<'a>,
        generated: ElementEnd<'a>,
    },
    InvalidElementTag {
        expected: ElementStart<'a>,
        generated: ElementStart<'a>,
    },
    ExpectedAttributesNotFound {
        expected: Vec<Attribute<'a>>,
        generated: Vec<Attribute<'a>>,
        difference: Vec<StrSpan<'a>>,
    },
    UnexpectedAttributesFound(Vec<StrSpan<'a>>),
    ExpectedAttributeNotFound {
        expected: Attribute<'a>,
    },
    InvalidAttributeValue {
        expected: Attribute<'a>,
        generated: Attribute<'a>,
    },
    ExpectedClassesNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    UnexpectedClassesFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    ExpectedStylesNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    UnexpectedStylesFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        difference: BTreeSet<&'a str>,
    },
    ExpectedStyleNotFound {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        missing: &'a str,
    },
    InvalidStyleValue {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        key: &'a str,
        expected_value: &'a str,
        generated_value: &'a str,
    },
    TextMismatch {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
    },
    CssMismatch {
        expected: StrSpan<'a>,
        generated: StrSpan<'a>,
        error: css_compare::Error<'a>,
    }
}

impl<'a> ErrorKind<'a> {
    pub fn display(&self) -> String {
        // TODO improve error display
        format!("{self:?}")
    }
}

#[derive(Debug)]
pub struct Error<'a> {
    pub expected: &'a str,
    pub generated: &'a str,
    pub kind: ErrorKind<'a>,
}

fn display_subset<'a>(data: &str, span: StrSpan<'a>, gap: usize) -> String {
    let start = span.start().checked_sub(gap).unwrap_or(0);
    let end = usize::min(span.end() + gap, data.len());
    format!(
        "{}{}{}",
        &data[start..span.start()],
        data[span.start()..span.end()].red().bold(),
        &data[span.end()..end]
    )
}

const SUBSET_GAP: usize = 150;

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::ElementMismatch {
                expected,
                generated,
            } => {
                writeln!(f, "= Element mismatch")?;
                writeln!(f, "== Expected result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.expected, expected.span(), SUBSET_GAP)
                )?;
                writeln!(f, "")?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.generated, generated.span(), SUBSET_GAP)
                )?;
                writeln!(f, "")?;
            }
            ErrorKind::TextMismatch {
                expected,
                generated,
            } => {
                writeln!(f, "= Element mismatch")?;
                writeln!(f, "== Expected result")?;
                writeln!(f, "{}", display_subset(self.expected, expected, SUBSET_GAP))?;
                writeln!(f, "")?;
                writeln!(f, "== Generated result")?;
                writeln!(
                    f,
                    "{}",
                    display_subset(self.generated, generated, SUBSET_GAP)
                )?;
                writeln!(f, "")?;
            }
            _ => {
                writeln!(f, "{:?}", self.kind)?;
            }
        }
        Ok(())
    }
}
