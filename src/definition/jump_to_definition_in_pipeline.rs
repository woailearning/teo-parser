use crate::ast::pipeline::Pipeline;
use crate::ast::schema::Schema;
use crate::ast::source::Source;
use crate::definition::definition::Definition;

pub(super) fn jump_to_definition_in_pipeline<'a>(
    schema: &'a Schema,
    source: &'a Source,
    pipeline: &'a Pipeline,
    namespace_path: &Vec<&'a str>,
    line_col: (usize, usize),
) -> Vec<Definition> {
    vec![]
}