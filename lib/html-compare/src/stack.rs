use htmlparser::{ElementEnd, Token, Tokenizer};
use std::collections::VecDeque;

fn is_head_style_element_start(token: Token<'_>) -> bool {
    if let Token::ElementStart { prefix, local, .. } = token {
        prefix.as_str().is_empty() && local.as_str() == "style"
    } else {
        false
    }
}

fn is_head_style_element_end(token: Token<'_>) -> bool {
    if let Token::ElementEnd {
        end: ElementEnd::Close(prefix, local),
        ..
    } = token
    {
        prefix.as_str().is_empty() && local.as_str() == "style"
    } else {
        false
    }
}

fn get_head_style_empty_closing<'a>(items: &VecDeque<Token<'a>>) -> Option<usize> {
    let mut index = 0;
    while let Some(item) = items.get(index) {
        match item {
            Token::ElementEnd {
                end: ElementEnd::Empty,
                ..
            } => {
                return Some(index + 1);
            }
            Token::ElementEnd {
                end: ElementEnd::Open,
                ..
            } if items
                .get(index + 1)
                .copied()
                .map(is_head_style_element_end)
                .unwrap_or(false) =>
            {
                return Some(index + 2)
            }
            _ => {}
        }
        index += 1;
    }
    None
}

pub struct TokenStack<'a> {
    pub inner: VecDeque<Token<'a>>,
}

impl<'a> TokenStack<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut inner = VecDeque::new();
        let mut tokenizer = Tokenizer::from(input);
        while let Some(Ok(token)) = tokenizer.next() {
            inner.push_back(token);
        }
        Self { inner }
    }

    fn sanitize_text(self) -> Self {
        Self {
            inner: self
                .inner
                .into_iter()
                .filter(|item| match item {
                    // removes empty comments
                    Token::Comment { text, .. } => {
                        !crate::helper::cleanup_text(text.as_str()).is_empty()
                    }
                    // removes empty text
                    Token::Text { text } => !crate::helper::cleanup_text(text.as_str()).is_empty(),
                    _ => true,
                })
                .collect(),
        }
    }

    fn sanitize_empty_head_style(mut self) -> Self {
        let mut inner = VecDeque::with_capacity(self.inner.len());
        while let Some(token) = self.inner.pop_front() {
            if is_head_style_element_start(token) {
                if let Some(count) = get_head_style_empty_closing(&self.inner) {
                    for _ in 0..count {
                        self.inner.pop_front();
                    }
                    continue;
                }
            }
            inner.push_back(token);
        }
        Self { inner }
    }

    fn sanitize_close_and_open_condition(mut self) -> Self {
        let mut inner = VecDeque::with_capacity(self.inner.len());
        let mut previous_condition = None;
        while let Some(token) = self.inner.pop_front() {
            match token {
                Token::ConditionalCommentStart { condition, .. } => {
                    previous_condition = Some(condition.as_str());
                    inner.push_back(token);
                }
                Token::ConditionalCommentEnd { .. } => {
                    match self.inner.pop_front() {
                        Some(Token::ConditionalCommentStart { condition, .. }) if Some(condition.as_str()) == previous_condition => {
                            // do nothing
                        }
                        Some(next) => {
                            self.inner.push_front(next);
                            inner.push_back(token);
                        }
                        None => {
                            inner.push_back(token);
                        }
                    }
                }
                other => {
                    inner.push_back(other);
                }
            }
        }
        Self { inner }
    }

    pub fn sanitize(self) -> Self {
        self.sanitize_text()
            .sanitize_close_and_open_condition()
            .sanitize_empty_head_style()
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        self.inner.pop_front()
    }

    pub fn head(&mut self) -> Option<Token<'a>> {
        self.inner.front().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplication_conditions() {
        let expected =
            TokenStack::parse("<br><!--[if mso | IE]></td><![endif]--><!--[if mso | IE]></tr>")
                .sanitize();
        let result = TokenStack::parse("<br><!--[if mso | IE]></td></tr>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn keep_different_conditions() {
        let expected = TokenStack::parse(
            "<first><!--[if mso]><second><![endif]--><!--[if lte mso 11]></third><![endif]-->",
        )
        .sanitize();
        let result = TokenStack::parse(
            "<first><!--[if mso]><second><![endif]--><!--[if lte mso 11]></third><![endif]-->",
        );
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn remove_head_style() {
        let expected = TokenStack::parse("<head></head>");
        let result = TokenStack::parse("<head><style></style></head>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn keep_head_style() {
        let expected = TokenStack::parse("<head><style>foo {}</style></head>");
        let result = TokenStack::parse("<head><style>foo {}</style></head>");
        assert_eq!(
            expected
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>(),
            result
                .sanitize()
                .inner
                .into_iter()
                .map(|token| token.span().as_str())
                .collect::<Vec<_>>()
        );
    }
}
