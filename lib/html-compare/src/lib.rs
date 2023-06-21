use std::collections::{BTreeMap, BTreeSet};
use xmlparser::{ElementEnd, StrSpan, Token, Tokenizer};

#[derive(Debug)]
pub enum Error {
    ExpectedElementNotFound(usize),
    UnexpectedElementFound(usize),
    ElementMismatch(usize, usize),
    EndOfElementMismatch(usize, usize),
    InvalidElementTag(usize, usize),
    ExpectedAttributesNotFound(Vec<String>),
    UnexpectedAttributesFound(Vec<String>),
    ExpectedAttributeNotFound(String),
    InvalidAttributeValue(String, String, String),
    ExpectedClassesNotFound(BTreeSet<String>),
    UnexpectedClassesFound(BTreeSet<String>),
    ExpectedStylesNotFound(BTreeSet<String>),
    UnexpectedStylesFound(BTreeSet<String>),
    ExpectedStyleNotFound(String),
    InvalidStyleValue(String, String, String),
    TextMismatch(usize, usize),
}

impl Error {
    pub fn display(&self) -> String {
        // TODO improve error display
        format!("{self:?}")
    }
}

// fn arround<'a>(text: &'a str, (start, end): (usize, usize), gap: usize) -> &'a str {
//     let real_start = start.checked_sub(gap).unwrap_or(0);
//     let real_end = usize::min(text.len(), end + gap);
//     &text[real_start..real_end]
// }

fn read_attributes<'a>(
    tokenizer: &mut Tokenizer<'a>,
) -> (BTreeMap<&'a str, &'a str>, ElementEnd<'a>, usize) {
    let mut result = BTreeMap::<&'a str, &'a str>::new();
    loop {
        match tokenizer.next() {
            Some(Ok(Token::Attribute { local, value, .. })) => {
                result.insert(local.as_str(), value.as_str());
            }
            Some(Ok(Token::ElementEnd { end, span })) => return (result, end, span.start()),
            _ => panic!("invalid token in attributes"),
        }
    }
}

struct Cursor<'a> {
    // expected_str: &'a str,
    expected_tokenizer: Tokenizer<'a>,
    // generated_str: &'a str,
    generated_tokenizer: Tokenizer<'a>,
}

impl<'a> Cursor<'a> {
    fn new(expected_str: &'a str, generated_str: &'a str) -> Self {
        Self {
            // expected_str,
            expected_tokenizer: Tokenizer::from(expected_str),
            // generated_str,
            generated_tokenizer: Tokenizer::from(generated_str),
        }
    }

    fn next(&mut self) -> (Option<Token<'a>>, Option<Token<'a>>) {
        (
            next_element(&mut self.expected_tokenizer),
            next_element(&mut self.generated_tokenizer),
        )
    }

    fn next_attributes(
        &mut self,
    ) -> (
        (BTreeMap<&str, &str>, xmlparser::ElementEnd<'a>, usize),
        (BTreeMap<&str, &str>, xmlparser::ElementEnd<'a>, usize),
    ) {
        (
            read_attributes(&mut self.expected_tokenizer),
            read_attributes(&mut self.generated_tokenizer),
        )
    }

    // fn arround(
    //     &self,
    //     (exp, gen): ((usize, usize), (usize, usize)),
    //     gap: usize,
    // ) -> (&'a str, &'a str) {
    //     (
    //         arround(self.expected_str, exp, gap),
    //         arround(self.generated_str, gen, gap),
    //     )
    // }
}

fn compare_classes(expected: &str, generated: &str) -> Result<(), Error> {
    let exp_values = expected
        .split(' ')
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();
    let res_values = generated
        .split(' ')
        .filter(|item| !item.is_empty())
        .collect::<BTreeSet<_>>();

    let diff = exp_values
        .difference(&res_values)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(Error::ExpectedClassesNotFound(diff));
    }

    let diff = res_values
        .difference(&exp_values)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(Error::UnexpectedClassesFound(diff));
    }

    Ok(())
}

fn compare_styles(expected: &str, generated: &str) -> Result<(), Error> {
    let exp_values = expected
        .split(';')
        .filter(|item| !item.is_empty())
        .filter_map(|item| item.split_once(':'))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect::<BTreeMap<_, _>>();
    let gen_values = generated
        .split(';')
        .filter(|item| !item.is_empty())
        .filter_map(|item| item.split_once(':'))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect::<BTreeMap<_, _>>();

    let exp_keys = exp_values.keys().cloned().collect::<BTreeSet<_>>();
    let res_keys = gen_values.keys().cloned().collect::<BTreeSet<_>>();

    let diff = exp_keys
        .difference(&res_keys)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(Error::ExpectedStylesNotFound(diff));
    }

    let diff = res_keys
        .difference(&exp_keys)
        .map(ToString::to_string)
        .collect::<BTreeSet<_>>();
    if !diff.is_empty() {
        return Err(Error::UnexpectedStylesFound(diff));
    }

    for (key, exp_value) in exp_values.iter() {
        if let Some(res_value) = gen_values.get(key) {
            if exp_value != res_value {
                return Err(Error::InvalidStyleValue(key.to_string(), exp_value.to_string(), res_value.to_string()));
            }
        } else {
            return Err(Error::ExpectedStyleNotFound(key.to_string()));
        }
    }

    Ok(())
}

fn compare_attributes<'a>(cursor: &mut Cursor<'a>) -> Result<(), Error> {
    let ((exp_attrs, exp_end, exp_end_pos), (gen_attrs, gen_end, gen_end_pos)) =
        cursor.next_attributes();
    
    if !exp_end.eq(&gen_end) {
        return Err(Error::EndOfElementMismatch(exp_end_pos, gen_end_pos));
    }
    
    let exp_keys = exp_attrs.keys().copied().collect::<BTreeSet<_>>();
    let gen_keys = gen_attrs.keys().copied().collect::<BTreeSet<_>>();

    let diff = exp_keys.difference(&gen_keys).map(ToString::to_string).collect::<Vec<_>>();
    if !diff.is_empty() {
        return Err(Error::ExpectedAttributesNotFound(diff));
    }
    let diff = gen_keys.difference(&exp_keys).map(ToString::to_string).collect::<Vec<_>>();
    if !diff.is_empty() {
        return Err(Error::UnexpectedAttributesFound(diff));
    }

    for (key, exp_value) in exp_attrs.iter() {
        if let Some(res_value) = gen_attrs.get(key) {
            if key == &"style" {
                compare_styles(exp_value, res_value)?;
            } else if key == &"class" {
                compare_classes(exp_value, &res_value)?;
            } else if exp_value != res_value {
                return Err(Error::InvalidAttributeValue(key.to_string(), exp_value.to_string(), res_value.to_string()));
            }
        } else {
            return Err(Error::ExpectedAttributeNotFound(key.to_string()));
        }
    }

    Ok(())
}

fn compare_elements<'a>(cursor: &mut Cursor<'a>, expected: StrSpan<'a>, generated: StrSpan<'a>) -> Result<(), Error> {
    if !expected.as_str().eq(generated.as_str()) {
        return Err(Error::InvalidElementTag(expected.start(), generated.start()));
    }

    compare_attributes(cursor)?;

    Ok(())
}

fn compare_tokens<'a>(cursor: &mut Cursor<'a>, expected: Token<'a>, generated: Token<'a>) -> Result<(), Error> {
    match (expected, generated) {
        (Token::Comment { text: exp_text, .. }, Token::Comment { text: res_text, .. }) => {
            compare_text(exp_text, res_text)?;
        }
        (Token::Text { text: exp_text }, Token::Text { text: res_text }) => {
            compare_text(exp_text, res_text)?;
        }
        (
            Token::ElementStart { span: exp_span, .. },
            Token::ElementStart { span: gen_span, .. },
        ) => {
            compare_elements(cursor, exp_span, gen_span)?;
        }
        (Token::ElementEnd { end: _exp_end, .. }, Token::ElementEnd { end: _res_end, .. }) => {
            // path.pop_back();
            // END OF ELEMENT
            return Ok(());
        }
        (exp, res) => {
            return Err(Error::ElementMismatch(token_position(exp).0, token_position(res).0));
        }
    }
    Ok(())
}

fn compare_next<'a>(cursor: &mut Cursor<'a>) -> Result<bool, Error> {
    match cursor.next() {
        (Some(expected), Some(generated)) => {
            compare_tokens(cursor, expected, generated)?;
            Ok(true)
        }
        (None, None) => {
            // nothing to do
            Ok(false)
        }
        (Some(token), None) => {
            let (start, _) = token_position(token);
            Err(Error::ExpectedElementNotFound(start))
        }
        (None, Some(token)) => {
            let (start, _) = token_position(token);
            Err(Error::UnexpectedElementFound(start))
        }
    }
}

fn cleanup_text(input: &str) -> String {
    input.replace([' ', '\t', '\n'], "")
}

fn trim_header_comment(input: &str) -> String {
    if input.starts_with("<!-- FILE:") {
        if let Some(index) = input.find('\n') {
            return input.split_at(index).1.to_string();
        }
    }
    input.to_string()
}

fn cleanup(input: &str) -> String {
    trim_header_comment(input)
        // conditions and comments
        // .replace(condition::END_NEGATION_CONDITIONAL_TAG, " ENDIF_NEGATION ")
        // .replace(condition::END_CONDITIONAL_TAG, " ENDIF ")
        // .replace(condition::START_MSO_NEGATION_CONDITIONAL_TAG, " IF_NO_MSO ")
        // .replace(
        //     condition::START_NEGATION_CONDITIONAL_TAG,
        //     " IF_NO_MSO_OR_IE ",
        // )
        .replace("<!--[if !mso><!-->", " IF_NO_MSO ")
        .replace("<!--[if !mso]><!-- -->", " IF_NO_MSO ")
        // .replace(condition::START_MSO_CONDITIONAL_TAG, " IF_MSO ")
        // .replace(condition::START_IE11_CONDITIONAL_TAG, " IF_LTE_MSO_11 ")
        // .replace(condition::START_CONDITIONAL_TAG, " IF_MSO_OR_IE ")
        // TODO handle removing closing and opening conditions
        .replace(" ENDIF  IF_MSO ", "")
        .replace(" ENDIF  IF_MSO_OR_IE ", "")
        // empty style header blocks
        .replace("<style type=\"text/css\">\n  </style>", "")
        // empty style attributes
        .replace("style=\"\"", "")
        // empty class attributes
        .replace("class=\"\"", "")
        // empty divs
        .replace("<div></div>", "")
        //
        .replace("<!doctype html>\n", "")
        .replace("<!doctype html>", "")
}

/// Compare html values without being too extreme
pub fn compare(expected: &str, generated: &str) -> Result<(), Error> {
    let expected = cleanup(expected);
    let generated = cleanup(generated);
    let mut cursor = Cursor::new(expected.as_str(), generated.as_str());
    while compare_next(&mut cursor)? {
        // nothing to do
    }
    Ok(())
}

fn span_position<'a>(span: StrSpan<'a>) -> (usize, usize) {
    (span.start(), span.end())
}

fn token_position<'a>(token: Token<'a>) -> (usize, usize) {
    span_position(match token {
        Token::Attribute { span, .. } => span,
        Token::Cdata { span, .. } => span,
        Token::Comment { span, .. } => span,
        Token::Declaration { span, .. } => span,
        Token::DtdEnd { span } => span,
        Token::DtdStart { span, .. } => span,
        Token::ElementEnd { span, .. } => span,
        Token::ElementStart { span, .. } => span,
        Token::EmptyDtd { span, .. } => span,
        Token::EntityDeclaration { span, .. } => span,
        Token::ProcessingInstruction { span, .. } => span,
        Token::Text { text } => text,
    })
}

fn next_element<'a>(tokenizer: &mut Tokenizer<'a>) -> Option<Token<'a>> {
    if let Some(token) = tokenizer.next() {
        let token = token.expect("unable to get next token");
        match token {
            Token::Text { text } => {
                if cleanup_text(text.as_str()).is_empty() {
                    next_element(tokenizer)
                } else {
                    Some(token)
                }
            }
            _ => Some(token),
        }
    } else {
        None
    }
}

fn compare_text<'a>(expected: StrSpan<'a>, result: StrSpan<'a>) -> Result<(), Error> {
    if cleanup_text(&expected) != cleanup_text(&result) {
        Err(Error::TextMismatch(expected.start(), result.start()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_same_classes() {
        compare_classes("foo bar baz", "baz foo bar").unwrap();
    }

    #[test]
    fn expected_class_not_found() {
        let err = compare_classes("foo bar baz", "baz bar").unwrap_err();
        assert_eq!(err.display(), "ExpectedClassesNotFound({\"foo\"})");
    }

    #[test]
    fn simple_same_styles() {
        compare_styles("width:100%;height:12px", "width: 100%; height: 12px;").unwrap();
    }

    #[test]
    fn expected_style_not_found() {
        let err = compare_styles("width:100%;height:12px", "width:100%").unwrap_err();
        assert_eq!(err.display(), "ExpectedStylesNotFound({\"height\"})");
    }

    #[test]
    fn simple_same_dom() {
        compare(
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"></html>
"#,
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"></html>
"#,
        ).unwrap();
        compare(
            r#"<!doctype html>
<html xmlns="http://www.w3.org/1999/xhtml"
    xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
</html>
"#,
            r#"<!doctype html>
<html  xmlns="http://www.w3.org/1999/xhtml" xmlns:o="urn:schemas-microsoft-com:office:office"  xmlns:v="urn:schemas-microsoft-com:vml">
</html>"#,
        ).unwrap();
    }
}
