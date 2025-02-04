use std::fmt::{Display, Formatter};
use crate::ast::expr::ExpressionKind;
use crate::ast::span::Span;

#[derive(Debug)]
pub(crate) struct Subscript {
    pub(crate) expression: Box<ExpressionKind>,
    pub(crate) span: Span,
}

impl Display for Subscript {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        Display::fmt(self.expression.as_ref(), f)?;
        f.write_str("]")
    }
}
