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

    pub fn sanitize(self) -> Self {
        self.sanitize_text().sanitize_empty_head_style()
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
