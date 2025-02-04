use crate::ast::arith::ArithExpr;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::definition::definition::Definition;

pub(super) fn jump_to_definition_in_arith_expr<'a>(
    schema: &'a Schema,
    source: &'a Source,
    arith_expr: &'a ArithExpr,
    namespace_path: &Vec<&'a str>,
    line_col: (usize, usize),
) -> Vec<Definition> {
    vec![]
}