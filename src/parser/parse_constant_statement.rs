use std::cell::RefCell;
use crate::ast::constant::Constant;
use crate::ast::expr::ExpressionKind;
use crate::ast::identifier::Identifier;
use crate::parser::parse_expression::parse_expression_kind;
use crate::parser::parse_identifier::parse_identifier;
use crate::parser::parse_span::parse_span;
use crate::parser::parse_type_expression::parse_type_expression;
use crate::parser::parser_context::ParserContext;
use crate::parser::pest_parser::{Pair, Rule};

pub(super) fn parse_constant_statement(pair: Pair<'_>, context: &mut ParserContext) -> Constant {
    let span = parse_span(&pair);
    let mut identifier: Option<Identifier> = None;
    let mut expression: Option<ExpressionKind> = None;
    let mut type_expr = None;
    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::identifier => identifier = Some(parse_identifier(&current)),
            Rule::expression => expression = Some(parse_expression_kind(current, context)),
            Rule::type_expression => type_expr = Some(parse_type_expression(current, context)),
            _ => context.insert_unparsed(parse_span(&current)),
        }
    }
    Constant {
        span,
        path: context.next_path(),
        string_path: context.next_string_path(identifier.as_ref().unwrap().name()),
        identifier: identifier.unwrap(),
        type_expr,
        expression: expression.unwrap(),
        resolved: RefCell::new(None),
    }
}
